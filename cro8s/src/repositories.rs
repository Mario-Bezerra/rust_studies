use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;

pub struct RustaceanRepository;

impl RustaceanRepository{
    
    pub fn find(conn : &mut PgConnection, id : i32) -> QueryResult<Rustacean>{
        rustaceans::table.find(id).get_result(conn)
    }
    
    pub fn find_multiple(conn : &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>>{
        rustaceans::table.limit(limit).load(conn)
    }
    
    pub fn create(conn : &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean>{
        diesel::insert_into(rustaceans::table)
                .values(new_rustacean)
                .get_result(conn)
    }
    
    pub fn update(conn : &mut PgConnection, id : i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
                .set((
                    rustaceans::name.eq(rustacean.name),
                    rustaceans::email.eq(rustacean.email)
                ))
                .get_result(conn)
    }
    
    pub fn delete(conn : &mut PgConnection, id : i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(conn)
    }
}

pub struct CrateRepository;

impl CrateRepository{
    
    pub fn find(conn : &mut PgConnection, id : i32) -> QueryResult<Crate>{
        crates::table.find(id).get_result(conn)
    }
    
    pub fn find_multiple(conn : &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>>{
        crates::table.limit(limit).load(conn)
    }
    
    pub fn create(conn : &mut PgConnection, new_crate : NewCrate) -> QueryResult<Crate>{
        diesel::insert_into(crates::table)
                .values(new_crate)
                .get_result(conn)
    }
    
    pub fn update(conn : &mut PgConnection, id : i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
                .set((
                    crates::rustacean_id.eq(a_crate.rustacean_id),
                    crates::code.eq(a_crate.code),
                    crates::name.eq(a_crate.name),
                    crates::version.eq(a_crate.version),
                    crates::description.eq(a_crate.description)
                ))
                .get_result(conn)
    }
    
    pub fn delete(conn : &mut PgConnection, id : i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn)
    }
}

pub struct UserRepository;

impl UserRepository {

    pub fn find_with_roles(conn: &mut PgConnection) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(conn).expect("Could not find users in method find_with_roles in UserREpository");
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(conn)
            .expect("Error loading User Roles in find_with_roles method")
            .grouped_by(&users);

        Ok(users.into_iter().zip(result).collect())
    }

    pub fn create(conn : &mut PgConnection, new_user : NewUser, roles_codes: Vec<String>) -> QueryResult<User>{
        let user = diesel::insert_into(users::table)
                .values(new_user)
                .get_result::<User>(conn)
                .expect("Error creating User in method create of UserRepository");

        for role_code in roles_codes {
            let new_user_role: NewUserRole = {
                    if let Ok(role) = RoleRepository::find_by_code(conn, role_code.to_owned()) {
                        NewUserRole {user_id: user.id, role_id: role.id}
                    } else {
                        let new_role = NewRole {code: role_code.to_owned(), name: role_code.to_owned()};
                        let role = RoleRepository::create(conn, new_role).expect("Error creating role");
                        NewUserRole {user_id: user.id, role_id: role.id}
                    }
            };
            let _ = diesel::insert_into(users_roles::table)
                    .values(new_user_role)
                    .get_result::<UserRole>(conn);
        }
        Ok(user)               
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        let _ = diesel::delete(
            users_roles::table.filter(users_roles::user_id.eq(id))
        ).execute(c);
        diesel::delete(users::table.find(id)).execute(c)
    }
}

pub struct RoleRepository;

impl RoleRepository {

    pub fn find_by_ids(conn: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids))
                        .load(conn)
    }

    pub fn find_by_code(conn: &mut PgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code))
            .first(conn)
    }

    pub fn find_by_user(conn: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user)
                                    .get_results::<UserRole>(conn)
                                    .expect("Could not find user role in method find_by_user in RoleRepository");
        
        let role_ids: Vec<i32> = user_roles.iter().map(|ur| ur.role_id).collect();

        Self::find_by_ids(conn, role_ids)
    }

    pub fn create(conn: &mut PgConnection, new_role : NewRole) -> QueryResult<Role>{
        diesel::insert_into(roles::table)
                .values(new_role)
                .get_result(conn)
    }
}