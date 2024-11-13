use anchor_lang::prelude::*;

use anchor_spl::token_interface::{transfer_checked, TransferChecked};

pub fn send_tokens<'info>(
    from: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    decimals: u8,
    amount: u64,
) -> Result<()> {
    let transfer_accounts = TransferChecked {
        from,
        mint,
        to,
        authority,
    };
    let cpi_context = CpiContext::new(token_program, transfer_accounts);
    transfer_checked(cpi_context, amount, decimals)
}
