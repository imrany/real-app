-- Add up migration script here
 create table book (
    title varchar not null,
    author varchar not null,
    isbn varchar not null
 );
 create unique index book_isbn_idx on book (isbn);

 create table users(
   username varchar(100) not null,
   email varchar(100) not null,
   pass_word varchar(24) not null 
 )
 create unique index email_idx on users (email)