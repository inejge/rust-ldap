// Demonstrates the Add operation.

use ldap3::result::Result;
use ldap3::LdapConn;
use maplit::hashset;

fn main() -> Result<()> {
    let mut ldap = LdapConn::new("ldap://localhost:2389")?;
    ldap.simple_bind("cn=Manager,dc=example,dc=org", "secret")?
        .success()?;
    let res = ldap
        .add(
            "uid=extra,ou=People,dc=example,dc=org",
            vec![
                ("objectClass", hashset! {"inetOrgPerson"}),
                ("uid", hashset! {"extra"}),
                ("cn", hashset! {"Extra User"}),
                ("sn", hashset! {"User"}),
            ],
        )?
        .success()?;
    println!("{:?}", res);
    Ok(ldap.unbind()?)
}
