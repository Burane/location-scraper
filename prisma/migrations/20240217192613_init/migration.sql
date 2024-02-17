-- CreateTable
CREATE TABLE "Model" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "price" REAL NOT NULL,
    "rooms" INTEGER NOT NULL,
    "area" REAL NOT NULL,
    "address" TEXT NOT NULL,
    "origin" TEXT NOT NULL,
    "dpe" TEXT,
    "scrapping_date" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE "Picture" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "url" TEXT NOT NULL,
    "modelId" INTEGER NOT NULL,
    CONSTRAINT "Picture_modelId_fkey" FOREIGN KEY ("modelId") REFERENCES "Model" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
