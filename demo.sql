CREATE TABLE fuck(
    id INTEGER PRIMARY_KEY AUTOINCREMENT,
    explains_id
);

CREATE TABLE explains(
    id INTEGER PRIMARY_KEY AUTOINCREAMENT,
    word_class VARCHAR NOT NULL,
    explain VARCHAR NOT NULL,
    fid INTEGER ,
    CONSTRAINT fk_fuck,
    FOREIGN KEY(fid),
    REFERENCES fuck(fid)
);

CREATE TABLE new_word(
       keyword TEXT PRIMARY_KEY,
       tips TEXT,
       level TEXT,
       phonetic_us TEXT,
       phonetic_uk TEXT,
       etymons TEXT,
);

CREATE TABLE "new_word" (
	"keyword"	TEXT NOT NULL,
	"tips"	TEXT,
	"level"	TEXT,
	"phonetic_us"	TEXT,
	"phonetic_uk"	TEXT,
	"etymons"	TEXT,
	PRIMARY KEY("keyword")
);

INSERT INTO "new_word" (
       "keyword",
       "tips",
       "level",
       "phonetic_us",
       "phonetic_uk",
       "etymons")
    VALUES
    ("hello",
    "音节划分：hel·lo",
    "海词5星基本词汇，属常用1000词。",
    "[hə`ləʊ]",
    "[hə`loʊ]",
    "☆ 1883年进入英语，直接源自古高地德语的hala，意为招呼船家用语。");


INSERT INTO "new_word" (
       "keyword",
       "tips",
       "level",
       "phonetic_us",
       "phonetic_uk",
       "etymons")
    VALUES
    ("bitch",
    "bitch",
    "bitch",
    "bitch",
    "[hə`loʊ]",
    "☆ 1883年进入英语，直接源自古高地德语的hala，意为招呼船家用语。");


CREATE TABLE "explains" (
	"part_of_speech"	TEXT NOT NULL,
	"keyword"	TEXT NOT NULL,
	"explain"	TEXT NOT NULL,
	FOREIGN KEY("keyword") REFERENCES "new_word"("keyword"),
	PRIMARY KEY("keyword")
);

INSERT INTO "explains"
    ("keyword",
    "part_of_speech",
    "explain")
VALUES
    ("hello",
    "n",
    "hello explains 2");

SELECT explains.keyword,
       explains.explain,
       explains.part_of_speech
       FROM explains
       INNER JOIN new_word
       ON explains.keyword = new_word.keyword;

SELECT explains.keyword,
       explains.explain,
       explains.part_of_speech
       FROM explains
       LEFT OUTER JOIN new_word
       ON explains.keyword = new_word.keyword;



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



CREATE TABLE "explains" (
    "keyword"   TEXT NOT NULL,
    "part_of_speech"    TEXT NOT NULL,
    "explain"   TEXT NOT NULL,
    FOREIGN KEY("keyword") REFERENCES "word"("keyword")
);



CREATE TABLE "distribution_data"(
    "keyword" TEXT NOT NULL,
    "frequency" INTEGER NOT NULL,
    "keyword_explain" TEXT NOT NULL,
);

SELECT explains.keyword,
       explains.explain,
       explains.part_of_speech
       FROM explains
       LEFT OUTER JOIN new_word
       ON explains.keyword = new_word.keyword;

SELECT word.*,
       explains.explain,
       explains.part_of_speech,
       distribution_data.frequency,
       distribution_data.keyword_explain
       FROM word
       INNER JOIN explains,distribution_data
       ON explains.keyword = word.keyword AND distribution_data.keyword = word.keyword;
