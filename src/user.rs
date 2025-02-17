enum Role{
    Admin,
    Users,
}
pub struct User{
    google_id: String,
    email: String,
    username: String,
    role: Role,
    created_at: NaiveDate,    
}
impl User{
    pub fn new(google_id_i: String,email_i: String,role_i: Role) -> User{
        //Code checks if google_id, username, or email already has accont
        //Throw error For each
        User {google_id:google_id_i, email: email_i, role: role_i,created_at: Utc::today().naive_utc()}
    }
}