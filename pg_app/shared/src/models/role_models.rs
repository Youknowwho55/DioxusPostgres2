use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::IntoEnumIterator;
use std::collections::HashSet;

// ===== Role and Permission Models =====

/// Defines the access levels and responsibilities for system users
///
/// # Database Representation
/// Stored as PostgreSQL enum type `user_role` with lowercase values
///
/// # Variants
/// - `Admin`: Full system access (superuser)
/// - `LoanOfficer`: Default role for loan processing staff
/// - `Processor`: Limited access role for data entry
///
/// # Examples
///
/// ## Database Usage
/// ```sql
/// CREATE TYPE user_role AS ENUM ('admin', 'loan_officer', 'processor');
/// ```
///
/// ## Rust Usage
/// ```rust
/// let role = UserRole::LoanOfficer;
/// assert_eq!(role.to_string(), "Loan Officer"); // Display formatting
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, strum::Display, strum::EnumIter, Default)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    /// System administrator with unrestricted access
    #[strum(serialize = "Administrator")]
    Admin,
    
    /// Loan officer with underwriting authority
    #[strum(serialize = "Loan Officer")]
    #[default]
    LoanOfficer,
    
    /// Loan processor with data entry permissions
    Processor,
}

/// Granular access controls for system functionality
///
/// Used in conjunction with `UserRole` to implement RBAC (Role-Based Access Control).
/// Permissions can be checked using `UserRole::permissions()`.
///
/// # Variant Capabilities
/// - `All`: Bypass all permission checks (admin only)
/// - `ViewLoans`: Read loan applications
/// - `CreateLoans`: Submit new loan applications  
/// - `EditOwnLoans`: Modify self-created loans
/// - `ProcessLoans`: Approve/reject applications
/// - `ManageUsers`: Create/modify user accounts
///
/// # Example Permission Check
/// ```rust
/// let required = Permission::CreateLoans;
/// let user_perms = user.get_permissions();
///
/// if user_perms.contains(&Permission::All) || user_perms.contains(&required) {
///     // Grant access
/// }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::EnumIter)]
pub enum Permission {
    /// Overrides all other permissions
    All,
   
    /// View loan applications
    ViewLoans,
    
    /// Create new loan applications
    CreateLoans,
    
    /// Edit loans created by self
    EditOwnLoans,
    
    /// Process loan applications (approve/reject)
    ProcessLoans,
    
    /// Manage user accounts and roles
    ManageUsers,
}

impl Permission {
    /// Common permission sets for quick checks
    pub fn loan_management() -> HashSet<Self> {
        [Self::ViewLoans, Self::CreateLoans, Self::ProcessLoans]
            .into_iter()
            .collect()
    }
}

impl UserRole {
    /// Converts from string (for CLI/config files)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(Self::Admin),
            "loan_officer" => Some(Self::LoanOfficer),
            "processor" => Some(Self::Processor),
            _ => None,
        }
    }

    /// Returns the set of permissions granted to this role
    ///
    /// # Example
    /// ```rust
    /// let officer_perms = UserRole::LoanOfficer.permissions();
    /// assert!(officer_perms.contains(&Permission::CreateLoans));
    /// ```
    pub fn permissions(&self) -> HashSet<Permission> {
        use Permission::*;
        
        match self {
            Self::Admin => Permission::iter().collect(),
            Self::LoanOfficer => [ViewLoans, CreateLoans, EditOwnLoans]
                .into_iter()
                .collect(),
            Self::Processor => [ViewLoans, ProcessLoans]
                .into_iter()
                .collect(),
        }
    }

    /// Checks if the role has a specific permission
    ///
    /// # Rules
    /// - Returns `true` if either:
    ///   - The permission is explicitly granted, or
    ///   - The role has `Permission::All` rights
    ///
    /// # Example
    /// ```rust
    /// assert!(UserRole::Admin.has_permission(Permission::ManageUsers));
    /// assert!(UserRole::LoanOfficer.has_permission(Permission::ViewLoans));
    /// ```
    pub fn has_permission(&self, permission: Permission) -> bool {
        let perms = self.permissions();
        perms.contains(&permission) || perms.contains(&Permission::All)
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| format!("Invalid role: {}", s))
    }
}