use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("BHcbEgJiscaaLHi3whdq8qZLCmEi1cDip953WsoZgQX8");

#[program]
pub mod cpi {
    use super::*;

    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info(); /// 获得 from 的pubKey
        let to_pubkey = ctx.accounts.recipient.to_account_info();/// 获得 to 的pubKey
        let program_id = ctx.accounts.system_program.to_account_info();/// 获得 自带的SOL转移程序ID:11111111111111111111111111111111

        let cpi_context = CpiContext::new(
            program_id,
            Transfer { /// 调用program 中的transfer 方法将SOL 从From转移到To
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>, /// 这里anchor 会自动校验这个sender 是不是 交易的签名pubKey
    #[account(mut)]
    recipient: SystemAccount<'info>, /// 前端传入 接收者的 pubKey
    system_program: Program<'info, System>,/// 系统自带的拥有SOL转移的程序 不需要传入参数
}


