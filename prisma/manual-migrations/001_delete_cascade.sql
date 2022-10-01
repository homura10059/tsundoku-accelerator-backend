alter table "EbookSnapshot"
drop constraint "EbookSnapshot_ebookId_fkey";

alter table "EbookSnapshot"
    add foreign key ("ebookId") references "Ebook"
        on update cascade on delete cascade;

alter table "EbookInWishList"
drop constraint "EbookInWishList_wishListId_fkey";

alter table "EbookInWishList"
    add foreign key ("wishListId") references "WishList"
        on update cascade on delete cascade;

