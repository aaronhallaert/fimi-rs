use fimi_magic::migrations;
mod helpers;

migrations!(
    _20241021212814_init_migration,
    _20241022110124_add_new_key_to_test_structs
);
