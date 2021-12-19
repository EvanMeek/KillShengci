CREATE TABLE "word"(
       "keyword" TEXT,
       "tips" TEXT,
       "level" TEXT,
       "phonetic_us" TEXT,
       "phonetic_uk" TEXT,
       "etymons" TEXT,
       "familiarity" TEXT,
       PRIMARY KEY("keyword")
);
INSERT INTO "word" VALUES
(

);


CREATE TABLE "explains" (
	"keyword"	TEXT NOT NULL,
	"part_of_speech"	TEXT NOT NULL,
	"explain"	TEXT NOT NULL,
	FOREIGN KEY("keyword") REFERENCES "word"("keyword"),
	PRIMARY KEY("keyword")
);
