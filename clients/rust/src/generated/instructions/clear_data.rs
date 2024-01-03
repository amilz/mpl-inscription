//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct ClearData {
    /// The account to store the metadata in.
    pub inscription_account: solana_program::pubkey::Pubkey,
    /// The account to store the inscription account's metadata in.
    pub inscription_metadata_account: solana_program::pubkey::Pubkey,
    /// The account that will pay for the transaction and rent.
    pub payer: solana_program::pubkey::Pubkey,
    /// The authority of the inscription account.
    pub authority: Option<solana_program::pubkey::Pubkey>,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
    /// The delegate record account.
    pub delegate_record: Option<solana_program::pubkey::Pubkey>,
}

impl ClearData {
    pub fn instruction(
        &self,
        args: ClearDataInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ClearDataInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.inscription_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.inscription_metadata_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        if let Some(authority) = self.authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                authority, true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_INSCRIPTION_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        if let Some(delegate_record) = self.delegate_record {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                delegate_record,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_INSCRIPTION_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = ClearDataInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct ClearDataInstructionData {
    discriminator: u8,
}

impl ClearDataInstructionData {
    fn new() -> Self {
        Self { discriminator: 4 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClearDataInstructionArgs {
    pub associated_tag: Option<String>,
}

/// Instruction builder.
#[derive(Default)]
pub struct ClearDataBuilder {
    inscription_account: Option<solana_program::pubkey::Pubkey>,
    inscription_metadata_account: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    delegate_record: Option<solana_program::pubkey::Pubkey>,
    associated_tag: Option<String>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ClearDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The account to store the metadata in.
    #[inline(always)]
    pub fn inscription_account(
        &mut self,
        inscription_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.inscription_account = Some(inscription_account);
        self
    }
    /// The account to store the inscription account's metadata in.
    #[inline(always)]
    pub fn inscription_metadata_account(
        &mut self,
        inscription_metadata_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.inscription_metadata_account = Some(inscription_metadata_account);
        self
    }
    /// The account that will pay for the transaction and rent.
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The authority of the inscription account.
    #[inline(always)]
    pub fn authority(&mut self, authority: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.authority = authority;
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account]`
    /// The delegate record account.
    #[inline(always)]
    pub fn delegate_record(
        &mut self,
        delegate_record: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.delegate_record = delegate_record;
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn associated_tag(&mut self, associated_tag: String) -> &mut Self {
        self.associated_tag = Some(associated_tag);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ClearData {
            inscription_account: self
                .inscription_account
                .expect("inscription_account is not set"),
            inscription_metadata_account: self
                .inscription_metadata_account
                .expect("inscription_metadata_account is not set"),
            payer: self.payer.expect("payer is not set"),
            authority: self.authority,
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            delegate_record: self.delegate_record,
        };
        let args = ClearDataInstructionArgs {
            associated_tag: self.associated_tag.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `clear_data` CPI accounts.
pub struct ClearDataCpiAccounts<'a, 'b> {
    /// The account to store the metadata in.
    pub inscription_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to store the inscription account's metadata in.
    pub inscription_metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account that will pay for the transaction and rent.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the inscription account.
    pub authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The delegate record account.
    pub delegate_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `clear_data` CPI instruction.
pub struct ClearDataCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to store the metadata in.
    pub inscription_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to store the inscription account's metadata in.
    pub inscription_metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account that will pay for the transaction and rent.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the inscription account.
    pub authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The delegate record account.
    pub delegate_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: ClearDataInstructionArgs,
}

impl<'a, 'b> ClearDataCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ClearDataCpiAccounts<'a, 'b>,
        args: ClearDataInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            inscription_account: accounts.inscription_account,
            inscription_metadata_account: accounts.inscription_metadata_account,
            payer: accounts.payer,
            authority: accounts.authority,
            system_program: accounts.system_program,
            delegate_record: accounts.delegate_record,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.inscription_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.inscription_metadata_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        if let Some(authority) = self.authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *authority.key,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_INSCRIPTION_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        if let Some(delegate_record) = self.delegate_record {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *delegate_record.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_INSCRIPTION_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = ClearDataInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.inscription_account.clone());
        account_infos.push(self.inscription_metadata_account.clone());
        account_infos.push(self.payer.clone());
        if let Some(authority) = self.authority {
            account_infos.push(authority.clone());
        }
        account_infos.push(self.system_program.clone());
        if let Some(delegate_record) = self.delegate_record {
            account_infos.push(delegate_record.clone());
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `clear_data` CPI instruction builder.
pub struct ClearDataCpiBuilder<'a, 'b> {
    instruction: Box<ClearDataCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClearDataCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClearDataCpiBuilderInstruction {
            __program: program,
            inscription_account: None,
            inscription_metadata_account: None,
            payer: None,
            authority: None,
            system_program: None,
            delegate_record: None,
            associated_tag: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The account to store the metadata in.
    #[inline(always)]
    pub fn inscription_account(
        &mut self,
        inscription_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.inscription_account = Some(inscription_account);
        self
    }
    /// The account to store the inscription account's metadata in.
    #[inline(always)]
    pub fn inscription_metadata_account(
        &mut self,
        inscription_metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.inscription_metadata_account = Some(inscription_metadata_account);
        self
    }
    /// The account that will pay for the transaction and rent.
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The authority of the inscription account.
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.authority = authority;
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// `[optional account]`
    /// The delegate record account.
    #[inline(always)]
    pub fn delegate_record(
        &mut self,
        delegate_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.delegate_record = delegate_record;
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn associated_tag(&mut self, associated_tag: String) -> &mut Self {
        self.instruction.associated_tag = Some(associated_tag);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = ClearDataInstructionArgs {
            associated_tag: self.instruction.associated_tag.clone(),
        };
        let instruction = ClearDataCpi {
            __program: self.instruction.__program,

            inscription_account: self
                .instruction
                .inscription_account
                .expect("inscription_account is not set"),

            inscription_metadata_account: self
                .instruction
                .inscription_metadata_account
                .expect("inscription_metadata_account is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            authority: self.instruction.authority,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            delegate_record: self.instruction.delegate_record,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ClearDataCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    inscription_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    inscription_metadata_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    delegate_record: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_tag: Option<String>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
