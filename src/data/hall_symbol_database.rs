use super::arithmetic_crystal_class::ArithmeticNumber;

pub type Number = i32;
pub type HallNumber = i32;
type HallSymbolEntry = (
    HallNumber,
    Number,
    ArithmeticNumber,
    &'static str, // setting
    &'static str, // Hall symbol
    &'static str, // HM symbol (short)
    &'static str, // HM symbol (full)
);

pub const HALL_SYMBOL_DATABASE: [HallSymbolEntry; 530] = [
    (1, 1, 1, "", "P 1", "P 1", "P 1"),
    (2, 2, 2, "", "-P 1", "P -1", "P -1"),
    (3, 3, 3, "b", "P 2y", "P 2", "P 1 2 1"),
    (4, 3, 3, "c", "P 2", "P 2", "P 1 1 2"),
    (5, 3, 3, "a", "P 2x", "P 2", "P 2 1 1"),
    (6, 4, 3, "b", "P 2yb", "P 2_1", "P 1 2_1 1"),
    (7, 4, 3, "c", "P 2c", "P 2_1", "P 1 1 2_1"),
    (8, 4, 3, "a", "P 2xa", "P 2_1", "P 2_1 1 1"),
    (9, 5, 4, "b1", "C 2y", "C 2", "C 1 2 1"),
    (10, 5, 4, "b2", "A 2y", "C 2", "A 1 2 1"),
    (11, 5, 4, "b3", "I 2y", "C 2", "I 1 2 1"),
    (12, 5, 4, "c1", "A 2", "C 2", "A 1 1 2"),
    (13, 5, 4, "c2", "B 2", "C 2", "B 1 1 2"),
    (14, 5, 4, "c3", "I 2", "C 2", "I 1 1 2"),
    (15, 5, 4, "a1", "B 2x", "C 2", "B 2 1 1"),
    (16, 5, 4, "a2", "C 2x", "C 2", "C 2 1 1"),
    (17, 5, 4, "a3", "I 2x", "C 2", "I 2 1 1"),
    (18, 6, 5, "b", "P -2y", "P m", "P 1 m 1"),
    (19, 6, 5, "c", "P -2", "P m", "P 1 1 m"),
    (20, 6, 5, "a", "P -2x", "P m", "P m 1 1"),
    (21, 7, 5, "b1", "P -2yc", "P c", "P 1 c 1"),
    (22, 7, 5, "b2", "P -2yac", "P c", "P 1 n 1"),
    (23, 7, 5, "b3", "P -2ya", "P c", "P 1 a 1"),
    (24, 7, 5, "c1", "P -2a", "P c", "P 1 1 a"),
    (25, 7, 5, "c2", "P -2ab", "P c", "P 1 1 n"),
    (26, 7, 5, "c3", "P -2b", "P c", "P 1 1 b"),
    (27, 7, 5, "a1", "P -2xb", "P c", "P b 1 1"),
    (28, 7, 5, "a2", "P -2xbc", "P c", "P n 1 1"),
    (29, 7, 5, "a3", "P -2xc", "P c", "P c 1 1"),
    (30, 8, 6, "b1", "C -2y", "C m", "C 1 m 1"),
    (31, 8, 6, "b2", "A -2y", "C m", "A 1 m 1"),
    (32, 8, 6, "b3", "I -2y", "C m", "I 1 m 1"),
    (33, 8, 6, "c1", "A -2", "C m", "A 1 1 m"),
    (34, 8, 6, "c2", "B -2", "C m", "B 1 1 m"),
    (35, 8, 6, "c3", "I -2", "C m", "I 1 1 m"),
    (36, 8, 6, "a1", "B -2x", "C m", "B m 1 1"),
    (37, 8, 6, "a2", "C -2x", "C m", "C m 1 1"),
    (38, 8, 6, "a3", "I -2x", "C m", "I m 1 1"),
    (39, 9, 6, "b1", "C -2yc", "C c", "C 1 c 1"),
    (40, 9, 6, "b2", "A -2yab", "C c", "A 1 n 1"),
    (41, 9, 6, "b3", "I -2ya", "C c", "I 1 a 1"),
    (42, 9, 6, "-b1", "A -2ya", "C c", "A 1 a 1"),
    (43, 9, 6, "-b2", "C -2yac", "C c", "C 1 n 1"),
    (44, 9, 6, "-b3", "I -2yc", "C c", "I 1 c 1"),
    (45, 9, 6, "c1", "A -2a", "C c", "A 1 1 a"),
    (46, 9, 6, "c2", "B -2ab", "C c", "B 1 1 n"),
    (47, 9, 6, "c3", "I -2b", "C c", "I 1 1 b"),
    (48, 9, 6, "-c1", "B -2b", "C c", "B 1 1 b"),
    (49, 9, 6, "-c2", "A -2ab", "C c", "A 1 1 n"),
    (50, 9, 6, "-c3", "I -2a", "C c", "I 1 1 a"),
    (51, 9, 6, "a1", "B -2xb", "C c", "B b 1 1"),
    (52, 9, 6, "a2", "C -2xac", "C c", "C n 1 1"),
    (53, 9, 6, "a3", "I -2xc", "C c", "I c 1 1"),
    (54, 9, 6, "-a1", "C -2xc", "C c", "C c 1 1"),
    (55, 9, 6, "-a2", "B -2xab", "C c", "B n 1 1"),
    (56, 9, 6, "-a3", "I -2xb", "C c", "I b 1 1"),
    (57, 10, 7, "b", "-P 2y", "P 2/m", "P 1 2/m 1"),
    (58, 10, 7, "c", "-P 2", "P 2/m", "P 1 1 2/m"),
    (59, 10, 7, "a", "-P 2x", "P 2/m", "P 2/m 1 1"),
    (60, 11, 7, "b", "-P 2yb", "P 2_1/m", "P 1 2_1/m 1"),
    (61, 11, 7, "c", "-P 2c", "P 2_1/m", "P 1 1 2_1/m"),
    (62, 11, 7, "a", "-P 2xa", "P 2_1/m", "P 2_1/m 1 1"),
    (63, 12, 8, "b1", "-C 2y", "C 2/m", "C 1 2/m 1"),
    (64, 12, 8, "b2", "-A 2y", "C 2/m", "A 1 2/m 1"),
    (65, 12, 8, "b3", "-I 2y", "C 2/m", "I 1 2/m 1"),
    (66, 12, 8, "c1", "-A 2", "C 2/m", "A 1 1 2/m"),
    (67, 12, 8, "c2", "-B 2", "C 2/m", "B 1 1 2/m"),
    (68, 12, 8, "c3", "-I 2", "C 2/m", "I 1 1 2/m"),
    (69, 12, 8, "a1", "-B 2x", "C 2/m", "B 2/m 1 1"),
    (70, 12, 8, "a2", "-C 2x", "C 2/m", "C 2/m 1 1"),
    (71, 12, 8, "a3", "-I 2x", "C 2/m", "I 2/m 1 1"),
    (72, 13, 7, "b1", "-P 2yc", "P 2/c", "P 1 2/c 1"),
    (73, 13, 7, "b2", "-P 2yac", "P 2/c", "P 1 2/n 1"),
    (74, 13, 7, "b3", "-P 2ya", "P 2/c", "P 1 2/a 1"),
    (75, 13, 7, "c1", "-P 2a", "P 2/c", "P 1 1 2/a"),
    (76, 13, 7, "c2", "-P 2ab", "P 2/c", "P 1 1 2/n"),
    (77, 13, 7, "c3", "-P 2b", "P 2/c", "P 1 1 2/b"),
    (78, 13, 7, "a1", "-P 2xb", "P 2/c", "P 2/b 1 1"),
    (79, 13, 7, "a2", "-P 2xbc", "P 2/c", "P 2/n 1 1"),
    (80, 13, 7, "a3", "-P 2xc", "P 2/c", "P 2/c 1 1"),
    (81, 14, 7, "b1", "-P 2ybc", "P 2_1/c", "P 1 2_1/c 1"),
    (82, 14, 7, "b2", "-P 2yn", "P 2_1/c", "P 1 2_1/n 1"),
    (83, 14, 7, "b3", "-P 2yab", "P 2_1/c", "P 1 2_1/a 1"),
    (84, 14, 7, "c1", "-P 2ac", "P 2_1/c", "P 1 1 2_1/a"),
    (85, 14, 7, "c2", "-P 2n", "P 2_1/c", "P 1 1 2_1/n"),
    (86, 14, 7, "c3", "-P 2bc", "P 2_1/c", "P 1 1 2_1/b"),
    (87, 14, 7, "a1", "-P 2xab", "P 2_1/c", "P 2_1/b 1 1"),
    (88, 14, 7, "a2", "-P 2xn", "P 2_1/c", "P 2_1/n 1 1"),
    (89, 14, 7, "a3", "-P 2xac", "P 2_1/c", "P 2_1/c 1 1"),
    (90, 15, 8, "b1", "-C 2yc", "C 2/c", "C 1 2/c 1"),
    (91, 15, 8, "b2", "-A 2yab", "C 2/c", "A 1 2/n 1"),
    (92, 15, 8, "b3", "-I 2ya", "C 2/c", "I 1 2/a 1"),
    (93, 15, 8, "-b1", "-A 2ya", "C 2/c", "A 1 2/a 1"),
    (94, 15, 8, "-b2", "-C 2yac", "C 2/c", "C 1 2/n 1"),
    (95, 15, 8, "-b3", "-I 2yc", "C 2/c", "I 1 2/c 1"),
    (96, 15, 8, "c1", "-A 2a", "C 2/c", "A 1 1 2/a"),
    (97, 15, 8, "c2", "-B 2ab", "C 2/c", "B 1 1 2/n"),
    (98, 15, 8, "c3", "-I 2b", "C 2/c", "I 1 1 2/b"),
    (99, 15, 8, "-c1", "-B 2b", "C 2/c", "B 1 1 2/b"),
    (100, 15, 8, "-c2", "-A 2ab", "C 2/c", "A 1 1 2/n"),
    (101, 15, 8, "-c3", "-I 2a", "C 2/c", "I 1 1 2/a"),
    (102, 15, 8, "a1", "-B 2xb", "C 2/c", "B 2/b 1 1"),
    (103, 15, 8, "a2", "-C 2xac", "C 2/c", "C 2/n 1 1"),
    (104, 15, 8, "a3", "-I 2xc", "C 2/c", "I 2/c 1 1"),
    (105, 15, 8, "-a1", "-C 2xc", "C 2/c", "C 2/c 1 1"),
    (106, 15, 8, "-a2", "-B 2xab", "C 2/c", "B 2/n 1 1"),
    (107, 15, 8, "-a3", "-I 2xb", "C 2/c", "I 2/b 1 1"),
    (108, 16, 9, "", "P 2 2", "P 2 2 2", "P 2 2 2"),
    (109, 17, 9, "", "P 2c 2", "P 2 2 2_1", "P 2 2 2_1"),
    (110, 17, 9, "cab", "P 2a 2a", "P 2_1 2 2", "P 2_1 2 2"),
    (111, 17, 9, "bca", "P 2 2b", "P 2 2_1 2", "P 2 2_1 2"),
    (112, 18, 9, "", "P 2 2ab", "P 2_1 2_1 2", "P 2_1 2_1 2"),
    (113, 18, 9, "cab", "P 2bc 2", "P 2 2_1 2_1", "P 2 2_1 2_1"),
    (114, 18, 9, "bca", "P 2ac 2ac", "P 2_1 2 2_1", "P 2_1 2 2_1"),
    (
        115,
        19,
        9,
        "",
        "P 2ac 2ab",
        "P 2_1 2_1 2_1",
        "P 2_1 2_1 2_1",
    ),
    (116, 20, 10, "", "C 2c 2", "C 2 2 2_1", "C 2 2 2_1"),
    (117, 20, 10, "cab", "A 2a 2a", "A 2_1 2 2", "A 2_1 2 2"),
    (118, 20, 10, "bca", "B 2 2b", "B 2 2_1 2", "B 2 2_1 2"),
    (119, 21, 10, "", "C 2 2", "C 2 2 2", "C 2 2 2"),
    (120, 21, 10, "cab", "A 2 2", "A 2 2 2", "A 2 2 2"),
    (121, 21, 10, "bca", "B 2 2", "B 2 2 2", "B 2 2 2"),
    (122, 22, 11, "", "F 2 2", "F 2 2 2", "F 2 2 2"),
    (123, 23, 12, "", "I 2 2", "I 2 2 2", "I 2 2 2"),
    (124, 24, 12, "", "I 2b 2c", "I 2_1 2_1 2_1", "I 2_1 2_1 2_1"),
    (125, 25, 13, "", "P 2 -2", "P m m 2", "P m m 2"),
    (126, 25, 13, "cab", "P -2 2", "P 2 m m", "P 2 m m"),
    (127, 25, 13, "bca", "P -2 -2", "P m 2 m", "P m 2 m"),
    (128, 26, 13, "", "P 2c -2", "P m c 2_1", "P m c 2_1"),
    (129, 26, 13, "ba-c", "P 2c -2c", "P c m 2_1", "P c m 2_1"),
    (130, 26, 13, "cab", "P -2a 2a", "P 2_1 m a", "P 2_1 m a"),
    (131, 26, 13, "-cba", "P -2 2a", "P 2_1 a m", "P 2_1 a m"),
    (132, 26, 13, "bca", "P -2 -2b", "P b 2_1 m", "P b 2_1 m"),
    (133, 26, 13, "a-cb", "P -2b -2", "P m 2_1 b", "P m 2_1 b"),
    (134, 27, 13, "", "P 2 -2c", "P c c 2", "P c c 2"),
    (135, 27, 13, "cab", "P -2a 2", "P 2 a a", "P 2 a a"),
    (136, 27, 13, "bca", "P -2b -2b", "P b 2 b", "P b 2 b"),
    (137, 28, 13, "", "P 2 -2a", "P m a 2", "P m a 2"),
    (138, 28, 13, "ba-c", "P 2 -2b", "P b m 2", "P b m 2"),
    (139, 28, 13, "cab", "P -2b 2", "P 2 m b", "P 2 m b"),
    (140, 28, 13, "-cba", "P -2c 2", "P 2 c m", "P 2 c m"),
    (141, 28, 13, "bca", "P -2c -2c", "P c 2 m", "P c 2 m"),
    (142, 28, 13, "a-cb", "P -2a -2a", "P m 2 a", "P m 2 a"),
    (143, 29, 13, "", "P 2c -2ac", "P c a 2_1", "P c a 2_1"),
    (144, 29, 13, "ba-c", "P 2c -2b", "P b c 2_1", "P b c 2_1"),
    (145, 29, 13, "cab", "P -2b 2a", "P 2_1 a b", "P 2_1 a b"),
    (146, 29, 13, "-cba", "P -2ac 2a", "P 2_1 c a", "P 2_1 c a"),
    (147, 29, 13, "bca", "P -2bc -2c", "P c 2_1 b", "P c 2_1 b"),
    (148, 29, 13, "a-cb", "P -2a -2ab", "P b 2_1 a", "P b 2_1 a"),
    (149, 30, 13, "", "P 2 -2bc", "P n c 2", "P n c 2"),
    (150, 30, 13, "ba-c", "P 2 -2ac", "P c n 2", "P c n 2"),
    (151, 30, 13, "cab", "P -2ac 2", "P 2 n a", "P 2 n a"),
    (152, 30, 13, "-cba", "P -2ab 2", "P 2 a n", "P 2 a n"),
    (153, 30, 13, "bca", "P -2ab -2ab", "P b 2 n", "P b 2 n"),
    (154, 30, 13, "a-cb", "P -2bc -2bc", "P n 2 b", "P n 2 b"),
    (155, 31, 13, "", "P 2ac -2", "P m n 2_1", "P m n 2_1"),
    (156, 31, 13, "ba-c", "P 2bc -2bc", "P n m 2_1", "P n m 2_1"),
    (157, 31, 13, "cab", "P -2ab 2ab", "P 2_1 m n", "P 2_1 m n"),
    (158, 31, 13, "-cba", "P -2 2ac", "P 2_1 n m", "P 2_1 n m"),
    (159, 31, 13, "bca", "P -2 -2bc", "P n 2_1 m", "P n 2_1 m"),
    (160, 31, 13, "a-cb", "P -2ab -2", "P m 2_1 n", "P m 2_1 n"),
    (161, 32, 13, "", "P 2 -2ab", "P b a 2", "P b a 2"),
    (162, 32, 13, "cab", "P -2bc 2", "P 2 c b", "P 2 c b"),
    (163, 32, 13, "bca", "P -2ac -2ac", "P c 2 a", "P c 2 a"),
    (164, 33, 13, "", "P 2c -2n", "P n a 2_1", "P n a 2_1"),
    (165, 33, 13, "ba-c", "P 2c -2ab", "P b n 2_1", "P b n 2_1"),
    (166, 33, 13, "cab", "P -2bc 2a", "P 2_1 n b", "P 2_1 n b"),
    (167, 33, 13, "-cba", "P -2n 2a", "P 2_1 c n", "P 2_1 c n"),
    (168, 33, 13, "bca", "P -2n -2ac", "P c 2_1 n", "P c 2_1 n"),
    (169, 33, 13, "a-cb", "P -2ac -2n", "P n 2_1 a", "P n 2_1 a"),
    (170, 34, 13, "", "P 2 -2n", "P n n 2", "P n n 2"),
    (171, 34, 13, "cab", "P -2n 2", "P 2 n n", "P 2 n n"),
    (172, 34, 13, "bca", "P -2n -2n", "P n 2 n", "P n 2 n"),
    (173, 35, 14, "", "C 2 -2", "C m m 2", "C m m 2"),
    (174, 35, 14, "cab", "A -2 2", "A 2 m m", "A 2 m m"),
    (175, 35, 14, "bca", "B -2 -2", "B m 2 m", "B m 2 m"),
    (176, 36, 14, "", "C 2c -2", "C m c 2_1", "C m c 2_1"),
    (177, 36, 14, "ba-c", "C 2c -2c", "C c m 2_1", "C c m 2_1"),
    (178, 36, 14, "cab", "A -2a 2a", "A 2_1 m a", "A 2_1 m a"),
    (179, 36, 14, "-cba", "A -2 2a", "A 2_1 a m", "A 2_1 a m"),
    (180, 36, 14, "bca", "B -2 -2b", "B b 2_1 m", "B b 2_1 m"),
    (181, 36, 14, "a-cb", "B -2b -2", "B m 2_1 b", "B m 2_1 b"),
    (182, 37, 14, "", "C 2 -2c", "C c c 2", "C c c 2"),
    (183, 37, 14, "cab", "A -2a 2", "A 2 a a", "A 2 a a"),
    (184, 37, 14, "bca", "B -2b -2b", "B b 2 b", "B b 2 b"),
    (185, 38, 15, "", "A 2 -2", "A m m 2", "A m m 2"),
    (186, 38, 15, "ba-c", "B 2 -2", "B m m 2", "B m m 2"),
    (187, 38, 15, "cab", "B -2 2", "B 2 m m", "B 2 m m"),
    (188, 38, 15, "-cba", "C -2 2", "C 2 m m", "C 2 m m"),
    (189, 38, 15, "bca", "C -2 -2", "C m 2 m", "C m 2 m"),
    (190, 38, 15, "a-cb", "A -2 -2", "A m 2 m", "A m 2 m"),
    (191, 39, 15, "", "A 2 -2b", "A e m 2", "A e m 2"),
    (192, 39, 15, "ba-c", "B 2 -2a", "B m e 2", "B m e 2"),
    (193, 39, 15, "cab", "B -2a 2", "B 2 e m", "B 2 e m"),
    (194, 39, 15, "-cba", "C -2a 2", "C 2 m e", "C 2 m e"),
    (195, 39, 15, "bca", "C -2a -2a", "C m 2 e", "C m 2 e"),
    (196, 39, 15, "a-cb", "A -2b -2b", "A e 2 m", "A e 2 m"),
    (197, 40, 15, "", "A 2 -2a", "A m a 2", "A m a 2"),
    (198, 40, 15, "ba-c", "B 2 -2b", "B b m 2", "B b m 2"),
    (199, 40, 15, "cab", "B -2b 2", "B 2 m b", "B 2 m b"),
    (200, 40, 15, "-cba", "C -2c 2", "C 2 c m", "C 2 c m"),
    (201, 40, 15, "bca", "C -2c -2c", "C c 2 m", "C c 2 m"),
    (202, 40, 15, "a-cb", "A -2a -2a", "A m 2 a", "A m 2 a"),
    (203, 41, 15, "", "A 2 -2ab", "A e a 2", "A e a 2"),
    (204, 41, 15, "ba-c", "B 2 -2ab", "B b e 2", "B b e 2"),
    (205, 41, 15, "cab", "B -2ab 2", "B 2 e b", "B 2 e b"),
    (206, 41, 15, "-cba", "C -2ac 2", "C 2 c e", "C 2 c e"),
    (207, 41, 15, "bca", "C -2ac -2ac", "C c 2 e", "C c 2 e"),
    (208, 41, 15, "a-cb", "A -2ab -2ab", "A e 2 a", "A e 2 a"),
    (209, 42, 16, "", "F 2 -2", "F m m 2", "F m m 2"),
    (210, 42, 16, "cab", "F -2 2", "F 2 m m", "F 2 m m"),
    (211, 42, 16, "bca", "F -2 -2", "F m 2 m", "F m 2 m"),
    (212, 43, 16, "", "F 2 -2d", "F d d 2", "F d d 2"),
    (213, 43, 16, "cab", "F -2d 2", "F 2 d d", "F 2 d d"),
    (214, 43, 16, "bca", "F -2d -2d", "F d 2 d", "F d 2 d"),
    (215, 44, 17, "", "I 2 -2", "I m m 2", "I m m 2"),
    (216, 44, 17, "cab", "I -2 2", "I 2 m m", "I 2 m m"),
    (217, 44, 17, "bca", "I -2 -2", "I m 2 m", "I m 2 m"),
    (218, 45, 17, "", "I 2 -2c", "I b a 2", "I b a 2"),
    (219, 45, 17, "cab", "I -2a 2", "I 2 c b", "I 2 c b"),
    (220, 45, 17, "bca", "I -2b -2b", "I c 2 a", "I c 2 a"),
    (221, 46, 17, "", "I 2 -2a", "I m a 2", "I m a 2"),
    (222, 46, 17, "ba-c", "I 2 -2b", "I b m 2", "I b m 2"),
    (223, 46, 17, "cab", "I -2b 2", "I 2 m b", "I 2 m b"),
    (224, 46, 17, "-cba", "I -2c 2", "I 2 c m", "I 2 c m"),
    (225, 46, 17, "bca", "I -2c -2c", "I c 2 m", "I c 2 m"),
    (226, 46, 17, "a-cb", "I -2a -2a", "I m 2 a", "I m 2 a"),
    (227, 47, 18, "", "-P 2 2", "P m m m", "P 2/m 2/m 2/m"),
    (228, 48, 18, "1", "P 2 2 -1n", "P n n n", "P 2/n 2/n 2/n"),
    (229, 48, 18, "2", "-P 2ab 2bc", "P n n n", "P 2/n 2/n 2/n"),
    (230, 49, 18, "", "-P 2 2c", "P c c m", "P 2/c 2/c 2/m"),
    (231, 49, 18, "cab", "-P 2a 2", "P m a a", "P 2/m 2/a 2/a"),
    (232, 49, 18, "bca", "-P 2b 2b", "P b m b", "P 2/b 2/m 2/b"),
    (233, 50, 18, "1", "P 2 2 -1ab", "P b a n", "P 2/b 2/a 2/n"),
    (234, 50, 18, "2", "-P 2ab 2b", "P b a n", "P 2/b 2/a 2/n"),
    (
        235,
        50,
        18,
        "1cab",
        "P 2 2 -1bc",
        "P n c b",
        "P 2/n 2/c 2/b",
    ),
    (236, 50, 18, "2cab", "-P 2b 2bc", "P n c b", "P 2/n 2/c 2/b"),
    (
        237,
        50,
        18,
        "1bca",
        "P 2 2 -1ac",
        "P c n a",
        "P 2/c 2/n 2/a",
    ),
    (238, 50, 18, "2bca", "-P 2a 2c", "P c n a", "P 2/c 2/n 2/a"),
    (239, 51, 18, "", "-P 2a 2a", "P m m a", "P 2_1/m 2/m 2/a"),
    (240, 51, 18, "ba-c", "-P 2b 2", "P m m b", "P 2/m 2_1/m 2/b"),
    (241, 51, 18, "cab", "-P 2 2b", "P b m m", "P 2/b 2_1/m 2/m"),
    (
        242,
        51,
        18,
        "-cba",
        "-P 2c 2c",
        "P c m m",
        "P 2/c 2/m 2_1/m",
    ),
    (243, 51, 18, "bca", "-P 2c 2", "P m c m", "P 2/m 2/c 2_1/m"),
    (244, 51, 18, "a-cb", "-P 2 2a", "P m a m", "P 2_1/m 2/a 2/m"),
    (245, 52, 18, "", "-P 2a 2bc", "P n n a", "P 2/n 2_1/n 2/a"),
    (
        246,
        52,
        18,
        "ba-c",
        "-P 2b 2n",
        "P n n b",
        "P 2_1/n 2/n 2/b",
    ),
    (247, 52, 18, "cab", "-P 2n 2b", "P b n n", "P 2/b 2/n 2_1/n"),
    (
        248,
        52,
        18,
        "-cba",
        "-P 2ab 2c",
        "P c n n",
        "P 2/c 2_1/n 2/n",
    ),
    (
        249,
        52,
        18,
        "bca",
        "-P 2ab 2n",
        "P n c n",
        "P 2_1/n 2/c 2/n",
    ),
    (
        250,
        52,
        18,
        "a-cb",
        "-P 2n 2bc",
        "P n a n",
        "P 2/n 2/a 2_1/n",
    ),
    (251, 53, 18, "", "-P 2ac 2", "P m n a", "P 2/m 2/n 2_1/a"),
    (
        252,
        53,
        18,
        "ba-c",
        "-P 2bc 2bc",
        "P n m b",
        "P 2/n 2/m 2_1/b",
    ),
    (
        253,
        53,
        18,
        "cab",
        "-P 2ab 2ab",
        "P b m n",
        "P 2_1/b 2/m 2/n",
    ),
    (
        254,
        53,
        18,
        "-cba",
        "-P 2 2ac",
        "P c n m",
        "P 2_1/c 2/n 2/m",
    ),
    (255, 53, 18, "bca", "-P 2 2bc", "P n c m", "P 2/n 2_1/c 2/m"),
    (
        256,
        53,
        18,
        "a-cb",
        "-P 2ab 2",
        "P m a n",
        "P 2/m 2_1/a 2/n",
    ),
    (257, 54, 18, "", "-P 2a 2ac", "P c c a", "P 2_1/c 2/c 2/a"),
    (
        258,
        54,
        18,
        "ba-c",
        "-P 2b 2c",
        "P c c b",
        "P 2/c 2_1/c 2/b",
    ),
    (259, 54, 18, "cab", "-P 2a 2b", "P b a a", "P 2/b 2_1/a 2/a"),
    (
        260,
        54,
        18,
        "-cba",
        "-P 2ac 2c",
        "P c a a",
        "P 2/c 2/a 2_1/a",
    ),
    (
        261,
        54,
        18,
        "bca",
        "-P 2bc 2b",
        "P b c b",
        "P 2/b 2/c 2_1/b",
    ),
    (
        262,
        54,
        18,
        "a-cb",
        "-P 2b 2ab",
        "P b a b",
        "P 2_1/b 2/a 2/b",
    ),
    (263, 55, 18, "", "-P 2 2ab", "P b a m", "P 2_1/b 2_1/a 2/m"),
    (
        264,
        55,
        18,
        "cab",
        "-P 2bc 2",
        "P m c b",
        "P 2/m 2_1/c 2_1/b",
    ),
    (
        265,
        55,
        18,
        "bca",
        "-P 2ac 2ac",
        "P c m a",
        "P 2_1/c 2/m 2_1/a",
    ),
    (
        266,
        56,
        18,
        "",
        "-P 2ab 2ac",
        "P c c n",
        "P 2_1/c 2_1/c 2/n",
    ),
    (
        267,
        56,
        18,
        "cab",
        "-P 2ac 2bc",
        "P n a a",
        "P 2/n 2_1/a 2_1/a",
    ),
    (
        268,
        56,
        18,
        "bca",
        "-P 2bc 2ab",
        "P b n b",
        "P 2_1/b 2/n 2_1/b",
    ),
    (269, 57, 18, "", "-P 2c 2b", "P b c m", "P 2/b 2_1/c 2_1/m"),
    (
        270,
        57,
        18,
        "ba-c",
        "-P 2c 2ac",
        "P c a m",
        "P 2_1/c 2/a 2_1/m",
    ),
    (
        271,
        57,
        18,
        "cab",
        "-P 2ac 2a",
        "P m c a",
        "P 2_1/m 2/c 2_1/a",
    ),
    (
        272,
        57,
        18,
        "-cba",
        "-P 2b 2a",
        "P m a b",
        "P 2_1/m 2_1/a 2/b",
    ),
    (
        273,
        57,
        18,
        "bca",
        "-P 2a 2ab",
        "P b m a",
        "P 2_1/b 2_1/m 2/a",
    ),
    (
        274,
        57,
        18,
        "a-cb",
        "-P 2bc 2c",
        "P c m b",
        "P 2/c 2_1/m 2_1/b",
    ),
    (275, 58, 18, "", "-P 2 2n", "P n n m", "P 2_1/n 2_1/n 2/m"),
    (
        276,
        58,
        18,
        "cab",
        "-P 2n 2",
        "P m n n",
        "P 2/m 2_1/n 2_1/n",
    ),
    (
        277,
        58,
        18,
        "bca",
        "-P 2n 2n",
        "P n m n",
        "P 2_1/n 2/m 2_1/n",
    ),
    (
        278,
        59,
        18,
        "1",
        "P 2 2ab -1ab",
        "P m m n",
        "P 2_1/m 2_1/m 2/n",
    ),
    (
        279,
        59,
        18,
        "2",
        "-P 2ab 2a",
        "P m m n",
        "P 2_1/m 2_1/m 2/n",
    ),
    (
        280,
        59,
        18,
        "1cab",
        "P 2bc 2 -1bc",
        "P n m m",
        "P 2/n 2_1/m 2_1/m",
    ),
    (
        281,
        59,
        18,
        "2cab",
        "-P 2c 2bc",
        "P n m m",
        "P 2/n 2_1/m 2_1/m",
    ),
    (
        282,
        59,
        18,
        "1bca",
        "P 2ac 2ac -1ac",
        "P m n m",
        "P 2_1/m 2/n 2_1/m",
    ),
    (
        283,
        59,
        18,
        "2bca",
        "-P 2c 2a",
        "P m n m",
        "P 2_1/m 2/n 2_1/m",
    ),
    (284, 60, 18, "", "-P 2n 2ab", "P b c n", "P 2_1/b 2/c 2_1/n"),
    (
        285,
        60,
        18,
        "ba-c",
        "-P 2n 2c",
        "P c a n",
        "P 2/c 2_1/a 2_1/n",
    ),
    (
        286,
        60,
        18,
        "cab",
        "-P 2a 2n",
        "P n c a",
        "P 2_1/n 2_1/c 2/a",
    ),
    (
        287,
        60,
        18,
        "-cba",
        "-P 2bc 2n",
        "P n a b",
        "P 2_1/n 2/a 2_1/b",
    ),
    (
        288,
        60,
        18,
        "bca",
        "-P 2ac 2b",
        "P b n a",
        "P 2/b 2_1/n 2_1/a",
    ),
    (
        289,
        60,
        18,
        "a-cb",
        "-P 2b 2ac",
        "P c n b",
        "P 2_1/c 2_1/n 2/b",
    ),
    (
        290,
        61,
        18,
        "",
        "-P 2ac 2ab",
        "P b c a",
        "P 2_1/b 2_1/c 2_1/a",
    ),
    (
        291,
        61,
        18,
        "ba-c",
        "-P 2bc 2ac",
        "P c a b",
        "P 2_1/c 2_1/a 2_1/b",
    ),
    (
        292,
        62,
        18,
        "",
        "-P 2ac 2n",
        "P n m a",
        "P 2_1/n 2_1/m 2_1/a",
    ),
    (
        293,
        62,
        18,
        "ba-c",
        "-P 2bc 2a",
        "P m n b",
        "P 2_1/m 2_1/n 2_1/b",
    ),
    (
        294,
        62,
        18,
        "cab",
        "-P 2c 2ab",
        "P b n m",
        "P 2_1/b 2_1/n 2_1/m",
    ),
    (
        295,
        62,
        18,
        "-cba",
        "-P 2n 2ac",
        "P c m n",
        "P 2_1/c 2_1/m 2_1/n",
    ),
    (
        296,
        62,
        18,
        "bca",
        "-P 2n 2a",
        "P m c n",
        "P 2_1/m 2_1/c 2_1/n",
    ),
    (
        297,
        62,
        18,
        "a-cb",
        "-P 2c 2n",
        "P n a m",
        "P 2_1/n 2_1/a 2_1/m",
    ),
    (298, 63, 19, "", "-C 2c 2", "C m c m", "C 2/m 2/c 2_1/m"),
    (
        299,
        63,
        19,
        "ba-c",
        "-C 2c 2c",
        "C c m m",
        "C 2/c 2/m 2_1/m",
    ),
    (300, 63, 19, "cab", "-A 2a 2a", "A m m a", "A 2_1/m 2/m 2/a"),
    (301, 63, 19, "-cba", "-A 2 2a", "A m a m", "A 2_1/m 2/a 2/m"),
    (302, 63, 19, "bca", "-B 2 2b", "B b m m", "B 2/b 2_1/m 2/m"),
    (303, 63, 19, "a-cb", "-B 2b 2", "B m m b", "B 2/m 2_1/m 2/b"),
    (304, 64, 19, "", "-C 2ac 2", "C m c e", "C 2/m 2/c 2_1/e"),
    (
        305,
        64,
        19,
        "ba-c",
        "-C 2ac 2ac",
        "C c m e",
        "C 2/c 2/m 2_1/e",
    ),
    (
        306,
        64,
        19,
        "cab",
        "-A 2ab 2ab",
        "A e m a",
        "A 2_1/e 2/m 2/a",
    ),
    (
        307,
        64,
        19,
        "-cba",
        "-A 2 2ab",
        "A e a m",
        "A 2_1/e 2/a 2/m",
    ),
    (308, 64, 19, "bca", "-B 2 2ab", "B b e m", "B 2/b 2_1/e 2/m"),
    (
        309,
        64,
        19,
        "a-cb",
        "-B 2ab 2",
        "B m e b",
        "B 2/m 2_1/e 2/b",
    ),
    (310, 65, 19, "", "-C 2 2", "C m m m", "C 2/m 2/m 2/m"),
    (311, 65, 19, "cab", "-A 2 2", "A m m m", "A 2/m 2/m 2/m"),
    (312, 65, 19, "bca", "-B 2 2", "B m m m", "B 2/m 2/m 2/m"),
    (313, 66, 19, "", "-C 2 2c", "C c c m", "C 2/c 2/c 2/m"),
    (314, 66, 19, "cab", "-A 2a 2", "A m a a", "A 2/m 2/a 2/a"),
    (315, 66, 19, "bca", "-B 2b 2b", "B b m b", "B 2/b 2/m 2/b"),
    (316, 67, 19, "", "-C 2a 2", "C m m e", "C 2/m 2/m 2/e"),
    (317, 67, 19, "ba-c", "-C 2a 2a", "C m m e", "C 2/m 2/m 2/e"),
    (318, 67, 19, "cab", "-A 2b 2b", "A e m m", "A 2/e 2/m 2/m"),
    (319, 67, 19, "-cba", "-A 2 2b", "A e m m", "A 2/e 2/m 2/m"),
    (320, 67, 19, "bca", "-B 2 2a", "B m e m", "B 2/m 2/e 2/m"),
    (321, 67, 19, "a-cb", "-B 2a 2", "B m e m", "B 2/m 2/e 2/m"),
    (322, 68, 19, "1", "C 2 2 -1ac", "C c c e", "C 2/c 2/c 2/e"),
    (323, 68, 19, "2", "-C 2a 2ac", "C c c e", "C 2/c 2/c 2/e"),
    (
        324,
        68,
        19,
        "1ba-c",
        "C 2 2 -1ac",
        "C c c e",
        "C 2/c 2/c 2/e",
    ),
    (325, 68, 19, "2ba-c", "-C 2a 2c", "C c c e", "C 2/c 2/c 2/e"),
    (
        326,
        68,
        19,
        "1cab",
        "A 2 2 -1ab",
        "A e a a",
        "A 2/e 2/a 2/a",
    ),
    (327, 68, 19, "2cab", "-A 2a 2b", "A e a a", "A 2/e 2/a 2/a"),
    (
        328,
        68,
        19,
        "1-cba",
        "A 2 2 -1ab",
        "A e a a",
        "A 2/e 2/a 2/a",
    ),
    (
        329,
        68,
        19,
        "2-cba",
        "-A 2ab 2b",
        "A e a a",
        "A 2/e 2/a 2/a",
    ),
    (
        330,
        68,
        19,
        "1bca",
        "B 2 2 -1ab",
        "B b e b",
        "B 2/b 2/e 2/b",
    ),
    (331, 68, 19, "2bca", "-B 2ab 2b", "B b c b", "B 2/b 2/e 2/b"),
    (
        332,
        68,
        19,
        "1a-cb",
        "B 2 2 -1ab",
        "B b e b",
        "B 2/b 2/e 2/b",
    ),
    (
        333,
        68,
        19,
        "2a-cb",
        "-B 2b 2ab",
        "B b e b",
        "B 2/b 2/e 2/b",
    ),
    (334, 69, 20, "", "-F 2 2", "F m m m", "F 2/m 2/m 2/m"),
    (335, 70, 20, "1", "F 2 2 -1d", "F d d d", "F 2/d 2/d 2/d"),
    (336, 70, 20, "2", "-F 2uv 2vw", "F d d d", "F 2/d 2/d 2/d"),
    (337, 71, 21, "", "-I 2 2", "I m m m", "I 2/m 2/m 2/m"),
    (338, 72, 21, "", "-I 2 2c", "I b a m", "I 2/b 2/a 2/m"),
    (339, 72, 21, "cab", "-I 2a 2", "I m c b", "I 2/m 2/c 2/b"),
    (340, 72, 21, "bca", "-I 2b 2b", "I c m a", "I 2/c 2/m 2/a"),
    (341, 73, 21, "", "-I 2b 2c", "I b c a", "I 2/b 2/c 2/a"),
    (342, 73, 21, "ba-c", "-I 2a 2b", "I c a b", "I 2/c 2/a 2/b"),
    (343, 74, 21, "", "-I 2b 2", "I m m a", "I 2/m 2/m 2/a"),
    (344, 74, 21, "ba-c", "-I 2a 2a", "I m m b", "I 2/m 2/m 2/b"),
    (345, 74, 21, "cab", "-I 2c 2c", "I b m m", "I 2/b 2/m 2/m"),
    (346, 74, 21, "-cba", "-I 2 2b", "I c m m", "I 2/c 2/m 2/m"),
    (347, 74, 21, "bca", "-I 2 2a", "I m c m", "I 2/m 2/c 2/m"),
    (348, 74, 21, "a-cb", "-I 2c 2", "I m a m", "I 2/m 2/a 2/m"),
    (349, 75, 22, "", "P 4", "P 4", "P 4"),
    (350, 76, 22, "", "P 4w", "P 4_1", "P 4_1"),
    (351, 77, 22, "", "P 4c", "P 4_2", "P 4_2"),
    (352, 78, 22, "", "P 4cw", "P 4_3", "P 4_3"),
    (353, 79, 23, "", "I 4", "I 4", "I 4"),
    (354, 80, 23, "", "I 4bw", "I 4_1", "I 4_1"),
    (355, 81, 24, "", "P -4", "P -4", "P -4"),
    (356, 82, 25, "", "I -4", "I -4", "I -4"),
    (357, 83, 26, "", "-P 4", "P 4/m", "P 4/m"),
    (358, 84, 26, "", "-P 4c", "P 4_2/m", "P 4_2/m"),
    (359, 85, 26, "1", "P 4ab -1ab", "P 4/n", "P 4/n"),
    (360, 85, 26, "2", "-P 4a", "P 4/n", "P 4/n"),
    (361, 86, 26, "1", "P 4n -1n", "P 4_2/n", "P 4_2/n"),
    (362, 86, 26, "2", "-P 4bc", "P 4_2/n", "P 4_2/n"),
    (363, 87, 27, "", "-I 4", "I 4/m", "I 4/m"),
    (364, 88, 27, "1", "I 4bw -1bw", "I 4_1/a", "I 4_1/a"),
    (365, 88, 27, "2", "-I 4ad", "I 4_1/a", "I 4_1/a"),
    (366, 89, 28, "", "P 4 2", "P 4 2 2", "P 4 2 2"),
    (367, 90, 28, "", "P 4ab 2ab", "P 4 2_1 2", "P 4 2_1 2"),
    (368, 91, 28, "", "P 4w 2c", "P 4_1 2 2", "P 4_1 2 2"),
    (369, 92, 28, "", "P 4abw 2nw", "P 4_1 2_1 2", "P 4_1 2_1 2"),
    (370, 93, 28, "", "P 4c 2", "P 4_2 2 2", "P 4_2 2 2"),
    (371, 94, 28, "", "P 4n 2n", "P 4_2 2_1 2", "P 4_2 2_1 2"),
    (372, 95, 28, "", "P 4cw 2c", "P 4_3 2 2", "P 4_3 2 2"),
    (373, 96, 28, "", "P 4nw 2abw", "P 4_3 2_1 2", "P 4_3 2_1 2"),
    (374, 97, 29, "", "I 4 2", "I 4 2 2", "I 4 2 2"),
    (375, 98, 29, "", "I 4bw 2bw", "I 4_1 2 2", "I 4_1 2 2"),
    (376, 99, 30, "", "P 4 -2", "P 4 m m", "P 4 m m"),
    (377, 100, 30, "", "P 4 -2ab", "P 4 b m", "P 4 b m"),
    (378, 101, 30, "", "P 4c -2c", "P 4_2 c m", "P 4_2 c m"),
    (379, 102, 30, "", "P 4n -2n", "P 4_2 n m", "P 4_2 n m"),
    (380, 103, 30, "", "P 4 -2c", "P 4 c c", "P 4 c c"),
    (381, 104, 30, "", "P 4 -2n", "P 4 n c", "P 4 n c"),
    (382, 105, 30, "", "P 4c -2", "P 4_2 m c", "P 4_2 m c"),
    (383, 106, 30, "", "P 4c -2ab", "P 4_2 b c", "P 4_2 b c"),
    (384, 107, 31, "", "I 4 -2", "I 4 m m", "I 4 m m"),
    (385, 108, 31, "", "I 4 -2c", "I 4 c m", "I 4 c m"),
    (386, 109, 31, "", "I 4bw -2", "I 4_1 m d", "I 4_1 m d"),
    (387, 110, 31, "", "I 4bw -2c", "I 4_1 c d", "I 4_1 c d"),
    (388, 111, 32, "", "P -4 2", "P -4 2 m", "P -4 2 m"),
    (389, 112, 32, "", "P -4 2c", "P -4 2 c", "P -4 2 c"),
    (390, 113, 32, "", "P -4 2ab", "P -4 2_1 m", "P -4 2_1 m"),
    (391, 114, 32, "", "P -4 2n", "P -4 2_1 c", "P -4 2_1 c"),
    (392, 115, 33, "", "P -4 -2", "P -4 m 2", "P -4 m 2"),
    (393, 116, 33, "", "P -4 -2c", "P -4 c 2", "P -4 c 2"),
    (394, 117, 33, "", "P -4 -2ab", "P -4 b 2", "P -4 b 2"),
    (395, 118, 33, "", "P -4 -2n", "P -4 n 2", "P -4 n 2"),
    (396, 119, 34, "", "I -4 -2", "I -4 m 2", "I -4 m 2"),
    (397, 120, 34, "", "I -4 -2c", "I -4 c 2", "I -4 c 2"),
    (398, 121, 35, "", "I -4 2", "I -4 2 m", "I -4 2 m"),
    (399, 122, 35, "", "I -4 2bw", "I -4 2 d", "I -4 2 d"),
    (400, 123, 36, "", "-P 4 2", "P 4/m m m", "P 4/m 2/m 2/m"),
    (401, 124, 36, "", "-P 4 2c", "P 4/m c c", "P 4/m 2/c 2/c"),
    (
        402,
        125,
        36,
        "1",
        "P 4 2 -1ab",
        "P 4/n b m",
        "P 4/n 2/b 2/m",
    ),
    (403, 125, 36, "2", "-P 4a 2b", "P 4/n b m", "P 4/n 2/b 2/m"),
    (404, 126, 36, "1", "P 4 2 -1n", "P 4/n n c", "P 4/n 2/n 2/c"),
    (405, 126, 36, "2", "-P 4a 2bc", "P 4/n n c", "P 4/n 2/n 2/c"),
    (406, 127, 36, "", "-P 4 2ab", "P 4/m b m", "P 4/m 2_1/b m"),
    (407, 128, 36, "", "-P 4 2n", "P 4/m n c", "P 4/m 2_1/n c"),
    (
        408,
        129,
        36,
        "1",
        "P 4ab 2ab -1ab",
        "P 4/n m m",
        "P 4/n 2_1/m m",
    ),
    (409, 129, 36, "2", "-P 4a 2a", "P 4/n m m", "P 4/n 2_1/m m"),
    (
        410,
        130,
        36,
        "1",
        "P 4ab 2n -1ab",
        "P 4/n c c",
        "P 4/n 2_1/c c",
    ),
    (411, 130, 36, "2", "-P 4a 2ac", "P 4/n c c", "P 4/n 2_1/c c"),
    (
        412,
        131,
        36,
        "",
        "-P 4c 2",
        "P 4_2/m m c",
        "P 4_2/m 2/m 2/c",
    ),
    (
        413,
        132,
        36,
        "",
        "-P 4c 2c",
        "P 4_2/m c m",
        "P 4_2/m 2/c 2/m",
    ),
    (
        414,
        133,
        36,
        "1",
        "P 4n 2c -1n",
        "P 4_2/n b c",
        "P 4_2/n 2/b 2/c",
    ),
    (
        415,
        133,
        36,
        "2",
        "-P 4ac 2b",
        "P 4_2/n b c",
        "P 4_2/n 2/b 2/c",
    ),
    (
        416,
        134,
        36,
        "1",
        "P 4n 2 -1n",
        "P 4_2/n n m",
        "P 4_2/n 2/n 2/m",
    ),
    (
        417,
        134,
        36,
        "2",
        "-P 4ac 2bc",
        "P 4_2/n n m",
        "P 4_2/n 2/n 2/m",
    ),
    (
        418,
        135,
        36,
        "",
        "-P 4c 2ab",
        "P 4_2/m b c",
        "P 4_2/m 2_1/b 2/c",
    ),
    (
        419,
        136,
        36,
        "",
        "-P 4n 2n",
        "P 4_2/m n m",
        "P 4_2/m 2_1/n 2/m",
    ),
    (
        420,
        137,
        36,
        "1",
        "P 4n 2n -1n",
        "P 4_2/n m c",
        "P 4_2/n 2_1/m 2/c",
    ),
    (
        421,
        137,
        36,
        "2",
        "-P 4ac 2a",
        "P 4_2/n m c",
        "P 4_2/n 2_1/m 2/c",
    ),
    (
        422,
        138,
        36,
        "1",
        "P 4n 2ab -1n",
        "P 4_2/n c m",
        "P 4_2/n 2_1/c 2/m",
    ),
    (
        423,
        138,
        36,
        "2",
        "-P 4ac 2ac",
        "P 4_2/n c m",
        "P 4_2/n 2_1/c 2/m",
    ),
    (424, 139, 37, "", "-I 4 2", "I 4/m m m", "I 4/m 2/m 2/m"),
    (425, 140, 37, "", "-I 4 2c", "I 4/m c m", "I 4/m 2/c 2/m"),
    (
        426,
        141,
        37,
        "1",
        "I 4bw 2bw -1bw",
        "I 4_1/a m d",
        "I 4_1/a 2/m 2/d",
    ),
    (
        427,
        141,
        37,
        "2",
        "-I 4bd 2",
        "I 4_1/a m d",
        "I 4_1/a 2/m 2/d",
    ),
    (
        428,
        142,
        37,
        "1",
        "I 4bw 2aw -1bw",
        "I 4_1/a c d",
        "I 4_1/a 2/c 2/d",
    ),
    (
        429,
        142,
        37,
        "2",
        "-I 4bd 2c",
        "I 4_1/a c d",
        "I 4_1/a 2/c 2/d",
    ),
    (430, 143, 38, "", "P 3", "P 3", "P 3"),
    (431, 144, 38, "", "P 31", "P 3_1", "P 3_1"),
    (432, 145, 38, "", "P 32", "P 3_2", "P 3_2"),
    (433, 146, 39, "H", "R 3", "R 3", "R 3"),
    (434, 146, 39, "R", "P 3*", "R 3", "R 3"),
    (435, 147, 40, "", "-P 3", "P -3", "P -3"),
    (436, 148, 41, "H", "-R 3", "R -3", "R -3"),
    (437, 148, 41, "R", "-P 3*", "R -3", "R -3"),
    (438, 149, 42, "", "P 3 2", "P 3 1 2", "P 3 1 2"),
    (439, 150, 43, "", "P 3 2=", "P 3 2 1", "P 3 2 1"),
    (440, 151, 42, "", "P 31 2 (0 0 4)", "P 3_1 1 2", "P 3_1 1 2"),
    (441, 152, 43, "", "P 31 2=", "P 3_1 2 1", "P 3_1 2 1"),
    (442, 153, 42, "", "P 32 2 (0 0 2)", "P 3_2 1 2", "P 3_2 1 2"),
    (443, 154, 43, "", "P 32 2=", "P 3_2 2 1", "P 3_2 2 1"),
    (444, 155, 44, "H", "R 3 2=", "R 3 2", "R 3 2"),
    (445, 155, 44, "R", "P 3* 2", "R 3 2", "R 3 2"),
    (446, 156, 45, "", "P 3 -2=", "P 3 m 1", "P 3 m 1"),
    (447, 157, 46, "", "P 3 -2", "P 3 1 m", "P 3 1 m"),
    (448, 158, 45, "", "P 3 -2=c", "P 3 c 1", "P 3 c 1"),
    (449, 159, 46, "", "P 3 -2c", "P 3 1 c", "P 3 1 c"),
    (450, 160, 47, "H", "R 3 -2=", "R 3 m", "R 3 m"),
    (451, 160, 47, "R", "P 3* -2", "R 3 m", "R 3 m"),
    (452, 161, 47, "H", "R 3 -2=c", "R 3 c", "R 3 c"),
    (453, 161, 47, "R", "P 3* -2n", "R 3 c", "R 3 c"),
    (454, 162, 48, "", "-P 3 2", "P -3 1 m", "P -3 1 2/m"),
    (455, 163, 48, "", "-P 3 2c", "P -3 1 c", "P -3 1 2/c"),
    (456, 164, 49, "", "-P 3 2=", "P -3 m 1", "P -3 2/m 1"),
    (457, 165, 49, "", "-P 3 2=c", "P -3 c 1", "P -3 2/c 1"),
    (458, 166, 50, "H", "-R 3 2=", "R -3 m", "R -3 2/m"),
    (459, 166, 50, "R", "-P 3* 2", "R -3 m", "R -3 2/m"),
    (460, 167, 50, "H", "-R 3 2=c", "R -3 c", "R -3 2/c"),
    (461, 167, 50, "R", "-P 3* 2n", "R -3 c", "R -3 2/c"),
    (462, 168, 51, "", "P 6", "P 6", "P 6"),
    (463, 169, 51, "", "P 61", "P 6_1", "P 6_1"),
    (464, 170, 51, "", "P 65", "P 6_5", "P 6_5"),
    (465, 171, 51, "", "P 62", "P 6_2", "P 6_2"),
    (466, 172, 51, "", "P 64", "P 6_4", "P 6_4"),
    (467, 173, 51, "", "P 6c", "P 6_3", "P 6_3"),
    (468, 174, 52, "", "P -6", "P -6", "P -6"),
    (469, 175, 53, "", "-P 6", "P 6/m", "P 6/m"),
    (470, 176, 53, "", "-P 6c", "P 6_3/m", "P 6_3/m"),
    (471, 177, 54, "", "P 6 2", "P 6 2 2", "P 6 2 2"),
    (472, 178, 54, "", "P 61 2 (0 0 5)", "P 6_1 2 2", "P 6_1 2 2"),
    (473, 179, 54, "", "P 65 2 (0 0 1)", "P 6_5 2 2", "P 6_5 2 2"),
    (474, 180, 54, "", "P 62 2 (0 0 4)", "P 6_2 2 2", "P 6_2 2 2"),
    (475, 181, 54, "", "P 64 2 (0 0 2)", "P 6_4 2 2", "P 6_4 2 2"),
    (476, 182, 54, "", "P 6c 2c", "P 6_3 2 2", "P 6_3 2 2"),
    (477, 183, 55, "", "P 6 -2", "P 6 m m", "P 6 m m"),
    (478, 184, 55, "", "P 6 -2c", "P 6 c c", "P 6 c c"),
    (479, 185, 55, "", "P 6c -2", "P 6_3 c m", "P 6_3 c m"),
    (480, 186, 55, "", "P 6c -2c", "P 6_3 m c", "P 6_3 m c"),
    (481, 187, 57, "", "P -6 2", "P -6 m 2", "P -6 m 2"),
    (482, 188, 57, "", "P -6c 2", "P -6 c 2", "P -6 c 2"),
    (483, 189, 56, "", "P -6 -2", "P -6 2 m", "P -6 2 m"),
    (484, 190, 56, "", "P -6c -2c", "P -6 2 c", "P -6 2 c"),
    (485, 191, 58, "", "-P 6 2", "P 6/m m m", "P 6/m 2/m 2/m"),
    (486, 192, 58, "", "-P 6 2c", "P 6/m c c", "P 6/m 2/c 2/c"),
    (
        487,
        193,
        58,
        "",
        "-P 6c 2",
        "P 6_3/m c m",
        "P 6_3/m 2/c 2/m",
    ),
    (
        488,
        194,
        58,
        "",
        "-P 6c 2c",
        "P 6_3/m m c",
        "P 6_3/m 2/m 2/c",
    ),
    (489, 195, 59, "", "P 2 2 3", "P 2 3", "P 2 3"),
    (490, 196, 60, "", "F 2 2 3", "F 2 3", "F 2 3"),
    (491, 197, 61, "", "I 2 2 3", "I 2 3", "I 2 3"),
    (492, 198, 59, "", "P 2ac 2ab 3", "P 2_1 3", "P 2_1 3"),
    (493, 199, 61, "", "I 2b 2c 3", "I 2_1 3", "I 2_1 3"),
    (494, 200, 62, "", "-P 2 2 3", "P m -3", "P 2/m -3"),
    (495, 201, 62, "1", "P 2 2 3 -1n", "P n -3", "P 2/n -3"),
    (496, 201, 62, "2", "-P 2ab 2bc 3", "P n -3", "P 2/n -3"),
    (497, 202, 63, "", "-F 2 2 3", "F m -3", "F 2/m -3"),
    (498, 203, 63, "1", "F 2 2 3 -1d", "F d -3", "F 2/d -3"),
    (499, 203, 63, "2", "-F 2uv 2vw 3", "F d -3", "F 2/d -3"),
    (500, 204, 64, "", "-I 2 2 3", "I m -3", "I 2/m -3"),
    (501, 205, 62, "", "-P 2ac 2ab 3", "P a -3", "P 2_1/a -3"),
    (502, 206, 64, "", "-I 2b 2c 3", "I a -3", "I 2_1/a -3"),
    (503, 207, 65, "", "P 4 2 3", "P 4 3 2", "P 4 3 2"),
    (504, 208, 65, "", "P 4n 2 3", "P 4_2 3 2", "P 4_2 3 2"),
    (505, 209, 66, "", "F 4 2 3", "F 4 3 2", "F 4 3 2"),
    (506, 210, 66, "", "F 4d 2 3", "F 4_1 3 2", "F 4_1 3 2"),
    (507, 211, 67, "", "I 4 2 3", "I 4 3 2", "I 4 3 2"),
    (508, 212, 65, "", "P 4acd 2ab 3", "P 4_3 3 2", "P 4_3 3 2"),
    (509, 213, 65, "", "P 4bd 2ab 3", "P 4_1 3 2", "P 4_1 3 2"),
    (510, 214, 67, "", "I 4bd 2c 3", "I 4_1 3 2", "I 4_1 3 2"),
    (511, 215, 68, "", "P -4 2 3", "P -4 3 m", "P -4 3 m"),
    (512, 216, 69, "", "F -4 2 3", "F -4 3 m", "F -4 3 m"),
    (513, 217, 70, "", "I -4 2 3", "I -4 3 m", "I -4 3 m"),
    (514, 218, 68, "", "P -4n 2 3", "P -4 3 n", "P -4 3 n"),
    (515, 219, 69, "", "F -4a 2 3", "F -4 3 c", "F -4 3 c"),
    (516, 220, 70, "", "I -4bd 2c 3", "I -4 3 d", "I -4 3 d"),
    (517, 221, 71, "", "-P 4 2 3", "P m -3 m", "P 4/m -3 2/m"),
    (518, 222, 71, "1", "P 4 2 3 -1n", "P n -3 n", "P 4/n -3 2/n"),
    (519, 222, 71, "2", "-P 4a 2bc 3", "P n -3 n", "P 4/n -3 2/n"),
    (520, 223, 71, "", "-P 4n 2 3", "P m -3 n", "P 4_2/m -3 2/n"),
    (
        521,
        224,
        71,
        "1",
        "P 4n 2 3 -1n",
        "P n -3 m",
        "P 4_2/n -3 2/m",
    ),
    (
        522,
        224,
        71,
        "2",
        "-P 4bc 2bc 3",
        "P n -3 m",
        "P 4_2/n -3 2/m",
    ),
    (523, 225, 72, "", "-F 4 2 3", "F m -3 m", "F 4/m -3 2/m"),
    (524, 226, 72, "", "-F 4a 2 3", "F m -3 c", "F 4/m -3 2/c"),
    (
        525,
        227,
        72,
        "1",
        "F 4d 2 3 -1d",
        "F d -3 m",
        "F 4_1/d -3 2/m",
    ),
    (
        526,
        227,
        72,
        "2",
        "-F 4vw 2vw 3",
        "F d -3 m",
        "F 4_1/d -3 2/m",
    ),
    (
        527,
        228,
        72,
        "1",
        "F 4d 2 3 -1ad",
        "F d -3 c",
        "F 4_1/d -3 2/c",
    ),
    (
        528,
        228,
        72,
        "2",
        "-F 4ud 2vw 3",
        "F d -3 c",
        "F 4_1/d -3 2/c",
    ),
    (529, 229, 73, "", "-I 4 2 3", "I m -3 m", "I 4/m -3 2/m"),
    (
        530,
        230,
        73,
        "",
        "-I 4bd 2c 3",
        "I a -3 d",
        "I 4_1/a -3 2/d",
    ),
];