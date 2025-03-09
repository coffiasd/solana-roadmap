/// 上下文类型提供了对以下非题词输入的访问的指令：
pub struct Context<'a, 'b, 'c, 'info, T: Bumps> {
    /// 当前执行的程序的pub地址
    pub program_id: &'a Pubkey,
    /// 供应帐户。
    pub accounts: &'b mut T,
    /// Remaining accounts 提供的未经验证的账号使用的时候必须非常小心
    pub remaining_accounts: &'c [AccountInfo<'info>],
    /// 在约束验证期间发现的生成的bumps种子。这是作为一个
    /// 提供的便利性，因此处理者不必重新计算生成的bumps种子，或者
    /// 将其作为参数传递。 
    /// type是＃[derive（eccounts）]生成的bumps struct
    pub bumps: T::Bumps,
}

