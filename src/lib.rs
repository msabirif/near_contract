use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};

use sha256::digest;
use chrono::Utc;

#[near_bindgen]
//#[derive(Default, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
//#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    projects: LookupMap<String, Project>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            projects: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn add_project(
        &mut self,
        project_name: String,
        logo: String,
        created_by: String,
    ) -> ProjectReturnMessage {
        let s_slice: &str = &project_name[..];
        let val = format!("{}{}", &logo, s_slice);
        let _hash = digest(val);

        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&_hash) {
            Some(p) => {
                return ProjectReturnMessage {
                    result: 409,
                    message: "Project already exists".to_owned(),
                    transaction_hash: transaction_hash,
                    hash: _hash,
                };
            }
            None => {
                self.projects.insert(
                    &_hash,
                    &Project::create_project(
                        created_by,
                        _hash.to_string(),
                        transaction_hash.to_string(),
                    ),
                );

                return ProjectReturnMessage {
                    result: 200,
                    message: "Project added successfully".to_owned(),
                    hash: _hash,
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_folder(
        &mut self,
        project_hash: String,
        project_id: String,
        folder_name: String,
    ) -> ReturnMessage {
        let s_slice: &str = &folder_name[..];
        let val = format!("{}{}", &project_id, s_slice);
        let _hash = digest(val);

        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.folders.iter().position(|r| r.folder_name == folder_name) {
                    Some(_index) => {
                        return ReturnMessage {
                            result: 409,
                            message: "Folder with the same name already exists".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        p.folders.push(Folder::create_folder(
                            _hash,
                            project_id,
                            folder_name,
                            transaction_hash.to_string(),
                        ));

                        self.projects.insert(&project_hash, &p);

                        return ReturnMessage {
                            result: 200,
                            message: "Folder added successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_sub_folder(
        &mut self,
        project_hash: String,
        project_id: String,
        folder_id: String,
        sub_folder_name: String,
    ) -> ReturnMessage {
        let s_slice: &str = &sub_folder_name[..];
        let val = format!("{}{}", &folder_id, s_slice);
        let _hash = digest(val);

        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p
                    .sub_folders
                    .iter()
                    .position(|r| r.sub_folder_name == sub_folder_name)
                {
                    Some(_index) => {
                        return ReturnMessage {
                            result: 409,
                            message: "Sub folder with the same name already exists".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        p.sub_folders.push(SubFolder::create_sub_folder(
                            _hash,
                            project_id,
                            folder_id,
                            sub_folder_name,
                            transaction_hash.to_string(),
                        ));

                        self.projects.insert(&project_hash, &p);

                        return ReturnMessage {
                            result: 200,
                            message: "Sub folder added successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_user(
        &mut self,
        project_hash: String,
        user_name: String,
        user_id: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.users.iter().position(|r| r.user_id == user_id) {
                    Some(_index) => {
                        return ReturnMessage {
                            result: 409,
                            message: "User already exists".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        p.users.push(User::create_user(
                            user_name,
                            user_id,
                            transaction_hash.to_string(),
                        ));

                        self.projects.insert(&project_hash, &p);

                        return ReturnMessage {
                            result: 200,
                            message: "User added successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_user_access(&mut self, project_hash: String, user_id: String) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.users.iter().position(|r| r.user_id == user_id) {
                    Some(_index) => {
                        if p.users[_index].is_revoked == true {
                            p.users[_index].is_revoked = false;

                            self.projects.insert(&project_hash, &p);
                        } else {
                            return ReturnMessage {
                                result: 200,
                                message: "User's access is already enabled".to_owned(),
                                transaction_hash: transaction_hash,
                            };
                        }

                        return ReturnMessage {
                            result: 200,
                            message: "User's access added successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        return ReturnMessage {
                            result: 404,
                            message: "User not found".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn remove_user_access(&mut self, project_hash: String, user_id: String) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.users.iter().position(|r| r.user_id == user_id) {
                    Some(_index) => {
                        if p.users[_index].is_revoked == false {
                            p.users[_index].is_revoked = true;

                            self.projects.insert(&project_hash, &p);
                        } else {
                            return ReturnMessage {
                                result: 200,
                                message: "User's access is already disabled".to_owned(),
                                transaction_hash: transaction_hash,
                            };
                        }

                        return ReturnMessage {
                            result: 200,
                            message: "User's access removed successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        return ReturnMessage {
                            result: 404,
                            message: "User not found".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_file(
        &mut self,
        project_hash: String,
        file_hash: String,
        title: String,
        user_id: String,
        folder_id: String,
        expiry_date: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(_index) => {
                        return ReturnMessage {
                            result: 409,
                            message: "File already exists".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        p.files.push(File::create_file(
                            file_hash,
                            title,
                            user_id,
                            folder_id,
                            expiry_date,
                            transaction_hash.to_string(),
                        ));

                        self.projects.insert(&project_hash, &p);

                        return ReturnMessage {
                            result: 200,
                            message: "File added successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn accept_file(&mut self, project_hash: String, file_hash: String) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(_index) => {
                        if p.files[_index].file_status != "GREEN" {
                            p.files[_index].file_status = "GREEN".to_owned();

                            self.projects.insert(&project_hash, &p);
                        } else {
                            return ReturnMessage {
                                result: 200,
                                message: "File status already accepted".to_owned(),
                                transaction_hash: transaction_hash,
                            };
                        }

                        return ReturnMessage {
                            result: 200,`
                            message: "File status accepted successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        return ReturnMessage {
                            result: 404,
                            message: "File not found".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn reject_file(&mut self, project_hash: String, file_hash: String) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(_index) => {
                        if p.files[_index].file_status != "RED" {
                            p.files[_index].file_status = "RED".to_owned();

                            self.projects.insert(&project_hash, &p);
                        } else {
                            return ReturnMessage {
                                result: 200,
                                message: "File status already rejected".to_owned(),
                                transaction_hash: transaction_hash,
                            };
                        }

                        return ReturnMessage {
                            result: 200,
                            message: "File status accepted successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        return ReturnMessage {
                            result: 404,
                            message: "File not found".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn update_file(
        &mut self,
        project_hash: String,
        file_hash: String,
        update_status: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(_index) => {
                        p.files[_index].file_status = update_status;

                        self.projects.insert(&project_hash, &p);

                        return ReturnMessage {
                            result: 200,
                            message: "File status updated successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        return ReturnMessage {
                            result: 404,
                            message: "File not found".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_validator(
        &mut self,
        project_hash: String,
        file_hash: String,
        validator_id: String,
        validator_ip: String,
        validator_email: String,
        validator_organization: String,
        can_sign: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(_index) => {
                        p.files[_index].validators.push(Validator::create_validator(
                            validator_id,
                            validator_ip,
                            validator_email,
                            validator_organization,
                            can_sign,
                            transaction_hash.to_string(),
                        ));

                        self.projects.insert(&project_hash, &p);

                        return ReturnMessage {
                            result: 200,
                            message: "File validator added successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        return ReturnMessage {
                            result: 409,
                            message: "File does not exist".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_validator_access(
        &mut self,
        project_hash: String,
        file_hash: String,
        validator_id: String,
        validator_email: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(f_index) => {
                        match p.files[f_index]
                            .validators
                            .iter()
                            .position(|r| r.validator_id == validator_id)
                        {
                            Some(_index) => {
                                if p.files[f_index].validators[_index].is_revoked == true {
                                    p.files[f_index].validators[_index].is_revoked = false;

                                    self.projects.insert(&project_hash, &p);
                                } else {
                                    return ReturnMessage {
                                        result: 200,
                                        message: "File validator's access is already enabled"
                                            .to_owned(),
                                        transaction_hash: transaction_hash,
                                    };
                                }

                                return ReturnMessage {
                                    result: 200,
                                    message: "File added successfully".to_owned(),
                                    transaction_hash: transaction_hash,
                                };
                            }
                            None => {
                                return ReturnMessage {
                                    result: 409,
                                    message: "File validator does not exist".to_owned(),
                                    transaction_hash: transaction_hash,
                                };
                            }
                        }
                    }
                    None => {
                        return ReturnMessage {
                            result: 409,
                            message: "File does not exist".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn remove_validator_access(
        &mut self,
        project_hash: String,
        file_hash: String,
        validator_id: String,
        validator_email: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(f_index) => {
                        match p.files[f_index]
                            .validators
                            .iter()
                            .position(|r| r.validator_id == validator_id)
                        {
                            Some(_index) => {
                                if p.files[f_index].validators[_index].is_revoked == false {
                                    p.files[f_index].validators[_index].is_revoked = true;

                                    self.projects.insert(&project_hash, &p);
                                } else {
                                    return ReturnMessage {
                                        result: 200,
                                        message: "File validator's access is already disabled"
                                            .to_owned(),
                                        transaction_hash: transaction_hash,
                                    };
                                }

                                return ReturnMessage {
                                    result: 200,
                                    message: "File added successfully".to_owned(),
                                    transaction_hash: transaction_hash,
                                };
                            }
                            None => {
                                return ReturnMessage {
                                    result: 409,
                                    message: "File validator does not exist".to_owned(),
                                    transaction_hash: transaction_hash,
                                };
                            }
                        }
                    }
                    None => {
                        return ReturnMessage {
                            result: 409,
                            message: "File does not exist".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn update_validator_after_file_validation(
        &mut self,
        project_hash: String,
        file_hash: String,
        validator_email: String,
        file_status: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p.files.iter().position(|r| r.file_hash == file_hash) {
                    Some(f_index) => {
                        match p.files[f_index]
                            .validators
                            .iter()
                            .position(|r| r.validator_email == validator_email)
                        {
                            Some(_index) => {
                                p.files[f_index].validators[_index].file_status = file_status;
                                //p.files[f_index].validators[_index].file_validation_hash = transaction_hash;

                                self.projects.insert(&project_hash, &p);

                                return ReturnMessage {
                                    result: 200,
                                    message: "File added successfully".to_owned(),
                                    transaction_hash: transaction_hash,
                                };
                            }
                            None => {
                                return ReturnMessage {
                                    result: 409,
                                    message: "File validator does not exist".to_owned(),
                                    transaction_hash: transaction_hash,
                                };
                            }
                        }
                    }
                    None => {
                        return ReturnMessage {
                            result: 409,
                            message: "File does not exist".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn query_project(&mut self, project_hash: String) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(p) => {
                return ReturnMessage {
                    result: 200,
                    message: "Project found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
            None => {
                return ReturnMessage {
                    result: 200,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }

    pub fn add_supplier(
        &mut self,
        project_hash: String,
        category: String,
        contact_name: String,
        supplier_id: String,
        supplier_email: String,
        company_name: String,
        company_website: String,
        requested_documents: String,
    ) -> ReturnMessage {
        let transaction_hash = "Transaction Hash".to_owned();

        match self.projects.get(&project_hash) {
            Some(mut p) => {
                let log_message = format!("Project found : {:?}", p);
                env::log(log_message.as_bytes());

                match p
                    .suppliers
                    .iter()
                    .position(|r| r.supplier_email == supplier_email)
                {
                    Some(_index) => {
                        return ReturnMessage {
                            result: 409,
                            message: "Supplier with the same email already exists".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                    None => {
                        p.suppliers.push(Supplier::create_supplier(
                            category,
                            contact_name,
                            supplier_id,
                            supplier_email,
                            company_name,
                            company_website,
                            requested_documents,
                            transaction_hash.to_string(),
                        ));

                        self.projects.insert(&project_hash, &p);

                        return ReturnMessage {
                            result: 200,
                            message: "Folder added successfully".to_owned(),
                            transaction_hash: transaction_hash,
                        };
                    }
                }
            }
            None => {
                return ReturnMessage {
                    result: 404,
                    message: "Project not found".to_owned(),
                    transaction_hash: transaction_hash,
                };
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ReturnMessage {
    pub result: u32,
    pub message: String,
    pub transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectReturnMessage {
    pub result: u32,
    pub message: String,
    pub transaction_hash: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateLogs {
    pub time_stamp: String,
    pub transaction_hash: String,
    pub transaction_type: String, //TransactionType::AddUserAccess("AddUserAccess"),
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Project {
    created_by: String,
    project_hash: String,
    folders: Vec<Folder>,
    sub_folders: Vec<SubFolder>,
    users: Vec<User>,
    files: Vec<File>,
    suppliers: Vec<Supplier>,
    update_logs: UpdateLogs,
}

impl Project {
    pub fn create_project(
        created_by: String,
        project_hash: String,
        transaction_hash: String,
    ) -> Self {
        Self {
            created_by,
            project_hash,
            folders: Vec::new(),
            sub_folders: Vec::new(),
            files: Vec::new(),
            suppliers: Vec::new(),
            users: Vec::new(),
            update_logs: UpdateLogs {
                time_stamp: Utc::now().to_string(),
                transaction_hash,
                transaction_type: "Add Project".to_owned(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Folder {
    folder_hash: String,
    project_id: String,
    folder_name: String,
    update_logs: UpdateLogs,
}

impl Folder {
    pub fn create_folder(
        folder_hash: String,
        project_id: String,
        folder_name: String,
        transaction_hash: String,
    ) -> Self {
        Self {
            folder_hash,
            project_id,
            folder_name,
            update_logs: UpdateLogs {
                time_stamp: Utc::now().to_string(),
                transaction_hash,
                transaction_type: "Add Folder".to_owned(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SubFolder {
    sub_folder_hash: String,
    project_id: String,
    folder_id: String,
    sub_folder_name: String,
    update_logs: UpdateLogs,
}

impl SubFolder {
    pub fn create_sub_folder(
        sub_folder_hash: String,
        project_id: String,
        folder_id: String,
        sub_folder_name: String,
        transaction_hash: String,
    ) -> Self {
        Self {
            sub_folder_hash,
            project_id,
            folder_id,
            sub_folder_name,
            update_logs: UpdateLogs {
                time_stamp: Utc::now().to_string(),
                transaction_hash,
                transaction_type: "Add Sub Folder".to_owned(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    user_name: String,
    user_id: String,
    is_revoked: bool,
    update_logs: UpdateLogs,
}

impl User {
    pub fn create_user(user_name: String, user_id: String, transaction_hash: String) -> Self {
        Self {
            user_name,
            user_id,
            is_revoked: false,
            update_logs: UpdateLogs {
                time_stamp: Utc::now().to_string(),
                transaction_hash,
                transaction_type: "Add User".to_owned(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct File {
    file_hash: String,
    file_title: String,
    user_id: String,
    folder_id: String,
    expiry_date: String,
    file_status: String,
    validators: Vec<Validator>,

    //user_organization: String,
    //user_email: String,
    //user_ip: String,
    //uploaded_at: String,
    update_logs: UpdateLogs,
}
/*
fileStatus = {
    GREEN: 'GREEN',
    AMBER: 'AMBER',
    RED: 'RED',
    GREY: 'GREY',
};
*/

impl File {
    pub fn create_file(
        file_hash: String,
        file_title: String,
        user_id: String,
        folder_id: String,
        expiry_date: String,
        transaction_hash: String,
    ) -> Self {
        Self {
            file_hash,
            file_title,
            user_id,
            folder_id,
            expiry_date,
            file_status: "RED".to_owned(),
            validators: Vec::new(),
            update_logs: UpdateLogs {
                time_stamp: Utc::now().to_string(),
                transaction_hash,
                transaction_type: "Add File".to_owned(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Validator {
    validator_id: String,
    validator_ip: String,
    validator_email: String,
    validator_organization: String,
    is_revoked: bool,
    can_sign: String, // Used hard coded and had purpose
    file_validation_hash: String,
    file_status: String,
    update_logs: UpdateLogs,
}

impl Validator {
    pub fn create_validator(
        validator_id: String,
        validator_ip: String,
        validator_email: String,
        validator_organization: String,
        can_sign: String, // Used hard coded and had purpose
        transaction_hash: String,
    ) -> Self {
        Self {
            validator_id,
            validator_ip,
            validator_email,
            validator_organization,
            can_sign,
            is_revoked: false,
            file_validation_hash: "".to_owned(),
            file_status: "".to_owned(),
            update_logs: UpdateLogs {
                time_stamp: Utc::now().to_string(),
                transaction_hash,
                transaction_type: "Add Validator".to_owned(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Supplier {
    category: String,
    contact_name: String,
    supplier_id: String,
    supplier_email: String,
    company_name: String,
    company_website: String,
    requested_documents: String,
    update_logs: UpdateLogs,
}

impl Supplier {
    pub fn create_supplier(
        category: String,
        contact_name: String,
        supplier_id: String,
        supplier_email: String,
        company_name: String,
        company_website: String,
        requested_documents: String,
        transaction_hash: String,
    ) -> Self {
        Self {
            category,
            contact_name,
            supplier_id,
            supplier_email,
            company_name,
            company_website,
            requested_documents,
            update_logs: UpdateLogs {
                time_stamp: Utc::now().to_string(),
                transaction_hash,
                transaction_type: "Add Supplier".to_owned(),
            },
        }
    }
}
