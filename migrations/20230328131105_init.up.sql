-- Add up migration script here
 create table book (
    title varchar not null,
    author varchar not null,
    isbn varchar not null
 );
 create unique index book_isbn_idx on book (isbn);

 -- Added users table
create table users (
   username varchar  not null,
   email varchar not null,
   pass_word varchar not null 
);
create unique index email_idx on users (email);

 