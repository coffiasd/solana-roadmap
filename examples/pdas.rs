#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + 1,   /// init,payer,space 需要一起使用，payer 指定了创建账号时的付款人，space为该空间指定为帐户分配的字节。
        seeds = [b"hello_world", signer.key().as_ref()], /// seeds 需要配合 bump一起使用。
        bump,
    )]
    pub pda_account: Account<'info, CustomAccount>,
    pub system_program: Program<'info, System>,
}

///这里定义的seeds b"hello_world" 会在生成IDL的时候体现在文件中。

#[account]
pub struct CustomAccount {
    pub bump_seed: u8,
}