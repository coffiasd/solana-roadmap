use anchor_lang::prelude::*;
 
declare_id!("11111111111111111111111111111111"); //@audit-info 定义了程序的链上地址，在build成功之后会被自动替换。
 
#[program] //@audit-info 属性注释包含程序的所有指令处理程序的模块。该模块中的每个公共功能都对应于可以调用的指令。
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
 
#[derive(Accounts)] //@audit-info 以指定调用指令时必须提供的帐户。该宏实现了帐户特征，这简化了帐户数据的验证和序列化和序列化。
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[account] //@audit-info 属性应用于定义程序创建的自定义帐户中存储的数据结构的结构。
pub struct NewAccount {
    data: u64,
}