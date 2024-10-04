use anchor_lang::prelude::*;

declare_id!("GEg5CBXRAVeHFmBbkyvbiQLWT8Zv3hvfrGt3og3bzv9E");


const MAX_MESSAGE_LEN: usize = 30;

const MAX_STUDENT_NAME: usize = 20;

#[program]
pub mod anchor_student_intro {


    use super::*;

    // creating the instruction logic for adding a student
    pub fn add_student_details(
        ctx: Context<AddStudent>,
        name: String,
        short_message: String,
    ) -> Result<()> {
        // make some require checks to ensure we're within boundary checks
        require!(name.len() <= MAX_STUDENT_NAME, StudentDetailError::NameTooLong);
        require!(short_message.len() <= MAX_MESSAGE_LEN, StudentDetailError::LongMessage);

        // log initial infos
        msg!("Adding Student Details to Records");
        msg!("Name of student is: {}", name);
        msg!("A little descriptive short message about student: {}", short_message);

        let student_details = &mut ctx.accounts.student_details;
        student_details.student = ctx.accounts.initializer.key();
        student_details.name = name;
        student_details.short_message = short_message;

        Ok(())
    }


    // we write the update instruction logic here:
    pub fn update_student_details(
        ctx: Context<UpdateStudent>,
        _name: String,
        short_message: String,
    ) -> Result<()> {
        // log the messages
        msg!("Updating the student details Processing");
        msg!("Short message description of student is updated to: {}", short_message);

        let student_details = &mut ctx.accounts.student_details;
        student_details.short_message = short_message;

        Ok(())
    }

    
    // we write the closing student account logic here
    pub fn close_student_details(_ctx: Context<DeleteStudent>, _name: String) -> Result<()> {
        // log the process
        msg!("Deleting student details from records!");

        Ok(())
    }
}





#[account]
#[derive(InitSpace)]
pub struct StudentIntroState {
    student: Pubkey,
#[max_len(20)]
    name: String,
#[max_len(30)]
    short_message: String,
}

const DISCRIMINATOR: usize = 8;


// creating custom errors to return on instruction check
#[error_code]
enum StudentDetailError {
    #[msg("Please provide 2 names, skip your middle name.")]
    NameTooLong,
    #[msg("Please make a short message about yourself no more than 30 characters")]
    LongMessage,
}

// addStudent struct
#[derive(Accounts)]
#[instruction(name: String, short_message: String)]
pub struct AddStudent<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key.as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + StudentIntroState::INIT_SPACE,
    )]
    pub student_details: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


// writing the updateStudent struct
#[derive(Accounts)]
#[instruction(name: String, short_message: String)]
pub struct UpdateStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key.as_ref()],
        bump,
        realloc = DISCRIMINATOR + StudentIntroState::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub student_details: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


// writing the delete Student struct
#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key.as_ref()],
        bump,
        close = initializer,
    )]
    pub student_details: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
