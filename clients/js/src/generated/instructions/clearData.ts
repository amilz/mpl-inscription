/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  option,
  string,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type ClearDataInstructionAccounts = {
  /** The account to store the metadata in. */
  inscriptionAccount: PublicKey | Pda;
  /** The account to store the inscription account's metadata in. */
  inscriptionMetadataAccount: PublicKey | Pda;
  /** The account that will pay for the transaction and rent. */
  payer?: Signer;
  /** The authority of the inscription account. */
  authority?: Signer;
  /** System program */
  systemProgram?: PublicKey | Pda;
  /** The delegate record account. */
  delegateRecord?: PublicKey | Pda;
};

// Data.
export type ClearDataInstructionData = {
  discriminator: number;
  associatedTag: Option<string>;
};

export type ClearDataInstructionDataArgs = {
  associatedTag: OptionOrNullable<string>;
};

export function getClearDataInstructionDataSerializer(): Serializer<
  ClearDataInstructionDataArgs,
  ClearDataInstructionData
> {
  return mapSerializer<
    ClearDataInstructionDataArgs,
    any,
    ClearDataInstructionData
  >(
    struct<ClearDataInstructionData>(
      [
        ['discriminator', u8()],
        ['associatedTag', option(string())],
      ],
      { description: 'ClearDataInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 4 })
  ) as Serializer<ClearDataInstructionDataArgs, ClearDataInstructionData>;
}

// Args.
export type ClearDataInstructionArgs = ClearDataInstructionDataArgs;

// Instruction.
export function clearData(
  context: Pick<Context, 'payer' | 'programs'>,
  input: ClearDataInstructionAccounts & ClearDataInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplInscription',
    '1NSCRfGeyo7wPUazGbaPBUsTM49e1k2aXewHGARfzSo'
  );

  // Accounts.
  const resolvedAccounts: ResolvedAccountsWithIndices = {
    inscriptionAccount: {
      index: 0,
      isWritable: true,
      value: input.inscriptionAccount ?? null,
    },
    inscriptionMetadataAccount: {
      index: 1,
      isWritable: true,
      value: input.inscriptionMetadataAccount ?? null,
    },
    payer: { index: 2, isWritable: true, value: input.payer ?? null },
    authority: { index: 3, isWritable: false, value: input.authority ?? null },
    systemProgram: {
      index: 4,
      isWritable: false,
      value: input.systemProgram ?? null,
    },
    delegateRecord: {
      index: 5,
      isWritable: false,
      value: input.delegateRecord ?? null,
    },
  };

  // Arguments.
  const resolvedArgs: ClearDataInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getClearDataInstructionDataSerializer().serialize(
    resolvedArgs as ClearDataInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
