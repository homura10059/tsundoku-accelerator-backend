-- CreateTable
CREATE TABLE "WishList" (
    "id" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "scrapedAt" INTEGER NOT NULL,
    "title" TEXT NOT NULL,

    CONSTRAINT "WishList_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Ebook" (
    "id" TEXT NOT NULL,
    "url" TEXT NOT NULL,

    CONSTRAINT "Ebook_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "EbookInWishList" (
    "wishListId" TEXT NOT NULL,
    "ebookId" TEXT NOT NULL,

    CONSTRAINT "EbookInWishList_pkey" PRIMARY KEY ("wishListId","ebookId")
);

-- CreateTable
CREATE TABLE "EbookSnapshot" (
    "id" TEXT NOT NULL,
    "ebookId" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "scrapedAt" INTEGER NOT NULL,
    "thumbnailUrl" TEXT NOT NULL,
    "price" INTEGER NOT NULL,
    "discount" INTEGER NOT NULL,
    "discountRate" INTEGER NOT NULL,
    "points" INTEGER NOT NULL,
    "pointsRate" INTEGER NOT NULL,

    CONSTRAINT "EbookSnapshot_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "WishList_url_key" ON "WishList"("url");

-- CreateIndex
CREATE UNIQUE INDEX "Ebook_url_key" ON "Ebook"("url");

-- AddForeignKey
ALTER TABLE "EbookInWishList" ADD CONSTRAINT "EbookInWishList_wishListId_fkey" FOREIGN KEY ("wishListId") REFERENCES "WishList"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "EbookInWishList" ADD CONSTRAINT "EbookInWishList_ebookId_fkey" FOREIGN KEY ("ebookId") REFERENCES "Ebook"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "EbookSnapshot" ADD CONSTRAINT "EbookSnapshot_ebookId_fkey" FOREIGN KEY ("ebookId") REFERENCES "Ebook"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
