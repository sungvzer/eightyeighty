| **Opcode (hex)** | **Instruction** | **size** |    **flags**    |                  **function**                  |
| :--------------: | :-------------: | :------: | :-------------: | :--------------------------------------------: |
|      **0**       |       NOP       |    1     |        -        |                       -                        |
|      **1**       |    LXI B,D16    |    3     |        -        |            B <- byte 3, C <- byte 2            |
|      **2**       |     STAX B      |    1     |        -        |                   (BC) <- A                    |
|      **3**       |      INX B      |    1     |        -        |                   BC <- BC+1                   |
|      **4**       |      INR B      |    1     |   Z, S, P, AC   |                    B <- B+1                    |
|      **5**       |      DCR B      |    1     |   Z, S, P, AC   |                    B <- B-1                    |
|      **6**       |    MVI B, D8    |    2     |        -        |                  B <- byte 2                   |
|      **7**       |       RLC       |    1     |       CY        | A = A «-1; bit 0 = prev bit 7; CY = prev bit 7 |
|      **8**       |        -        |    -     |        -        |                       -                        |
|      **9**       |      DAD B      |    1     |       CY        |                  HL = HL + BC                  |
|      **0a**      |     LDAX B      |    1     |        -        |                   A <- (BC)                    |
|      **0b**      |      DCX B      |    1     |        -        |                   BC = BC-1                    |
|      **0c**      |      INR C      |    1     |   Z, S, P, AC   |                    C <- C+1                    |
|      **0d**      |      DCR C      |    1     |   Z, S, P, AC   |                    C <-C-1                     |
|      **0e**      |    MVI C,D8     |    2     |        -        |                  C <- byte 2                   |
|      **0f**      |       RRC       |    1     |       CY        | A = A-» 1; bit 7 = prev bit 0; CY = prev bit 0 |
|      **0A**      |        -        |    -     |        -        |                       -                        |
|      **0B**      |    LXI D,D16    |    3     |        -        |            D <- byte 3, E <- byte 2            |
|      **0C**      |     STAX D      |    1     |        -        |                   (DE) <- A                    |
|      **0D**      |      INX D      |    1     |        -        |                  DE <- DE + 1                  |
|      **0E**      |      INR D      |    1     |   Z, S, P, AC   |                    D <- D+1                    |
|      **0F**      |      DCR D      |    1     |   Z, S, P, AC   |                    D <- D-1                    |
|      **10**      |    MVI D, D8    |    2     |        -        |                  D <- byte 2                   |
|      **11**      |       RAL       |    1     |       CY        |  A = A «-1; bit 0 = prev CY; CY = prev bit 7   |
|      **12**      |        -        |    -     |        -        |                       -                        |
|      **13**      |      DAD D      |    1     |       CY        |                  HL = HL + DE                  |
|      **1a**      |     LDAX D      |    1     |        -        |                   A <- (DE)                    |
|      **1b**      |      DCX D      |    1     |        -        |                   DE = DE-1                    |
|      **1c**      |      INR E      |    1     |   Z, S, P, AC   |                    E <-E+1                     |
|      **1d**      |      DCR E      |    1     |   Z, S, P, AC   |                    E <- E-1                    |
|      **1e**      |    MVI E,D8     |    2     |        -        |                  E <- byte 2                   |
|      **1f**      |       RAR       |    1     |       CY        | A = A-» 1; bit 7 = prev bit 7; CY = prev bit 0 |
|      **14**      |        -        |    -     |        -        |                       -                        |
|      **15**      |    LXI H,D16    |    3     |        -        |            H <- byte 3, L <- byte 2            |
|      **16**      |    SHLD adr     |    3     |        -        |             (adr) <-L; (adr+1)<-H              |
|      **17**      |      INX H      |    1     |        -        |                  HL <- HL + 1                  |
|      **18**      |      INR H      |    1     |   Z, S, P, AC   |                    H <- H+1                    |
|      **19**      |      DCR H      |    1     |   Z, S, P, AC   |                    H <- H-1                    |
|      **1A**      |    MVI H,D8     |    2     |        -        |                  H <- byte 2                   |
|      **1B**      |       DAA       |    1     |        -        |                    special                     |
|      **1C**      |        -        |    -     |        -        |                       -                        |
|      **1D**      |      DAD H      |    1     |       CY        |                  HL = HL + HI                  |
|      **2a**      |    LHLD adr     |    3     |        -        |             L <- (adr); H<-(adr+1)             |
|      **2b**      |      DCX H      |    1     |        -        |                   HL = HL-1                    |
|      **2c**      |      INR L      |    1     |   Z, S, P, AC   |                    L <- L+1                    |
|      **2d**      |      DCR L      |    1     |   Z, S, P, AC   |                    L <- L-1                    |
|      **2e**      |    MVI L, D8    |    2     |        -        |                  L <- byte 2                   |
|      **2f**      |       CMA       |    1     |        -        |                    A <- !A                     |
|      **1E**      |        -        |    -     |        -        |                       -                        |
|      **1F**      |   LXI SP, D16   |    3     |        -        |        SP.hi <- byte 3, SP.lo <- byte 2        |
|      **20**      |     STA adr     |    3     |        -        |                   (adr) <- A                   |
|      **21**      |     INX SP      |    1     |        -        |                  SP = SP + 1                   |
|      **22**      |      INR M      |    1     |   Z, S, P, AC   |                 (HL) <- (HL)+1                 |
|      **23**      |      DCR M      |    1     |   Z, S, P, AC   |                 (HL) <- (HL)-1                 |
|      **24**      |    MVI M,D8     |    2     |        -        |                 (HL) <- byte 2                 |
|      **25**      |       STC       |    1     |       CY        |                     CY = 1                     |
|      **26**      |        -        |    -     |        -        |                       -                        |
|      **27**      |     DAD SP      |    1     |       CY        |                  HL = HL + SP                  |
|      **3a**      |     LDA adr     |    3     |        -        |                   A <- (adr)                   |
|      **3b**      |     DCX SP      |    1     |        -        |                   SP = SP-1                    |
|      **3c**      |      INR A      |    1     |   Z, S, P, AC   |                    A <- A+1                    |
|      **3d**      |      DCR A      |    1     |   Z, S, P, AC   |                    A <- A-1                    |
|      **3e**      |    MVI A,D8     |    2     |        -        |                  A <- byte 2                   |
|      **3f**      |       CMC       |    1     |       CY        |                     CY=!CY                     |
|      **28**      |     MOV B,B     |    1     |        -        |                     B <- B                     |
|      **29**      |     MOV B,C     |    1     |        -        |                     B <- C                     |
|      **2A**      |     MOV B,D     |    1     |        -        |                     B <- D                     |
|      **2B**      |     MOV B,E     |    1     |        -        |                     B <- E                     |
|      **2C**      |     MOV B,H     |    1     |        -        |                     B <- H                     |
|      **2D**      |     MOV B,L     |    1     |        -        |                     B <- L                     |
|      **2E**      |     MOV B,M     |    1     |        -        |                   B <- (HL)                    |
|      **2F**      |     MOV B,A     |    1     |        -        |                     B <- A                     |
|      **30**      |     MOV C,B     |    1     |        -        |                     C <- B                     |
|      **31**      |     MOV C,C     |    1     |        -        |                     C <- C                     |
|      **4a**      |     MOV C,D     |    1     |        -        |                     C <- D                     |
|      **4b**      |     MOV C,E     |    1     |        -        |                     C <- E                     |
|      **4c**      |     MOV C,H     |    1     |        -        |                     C <- H                     |
|      **4d**      |     MOV C,L     |    1     |        -        |                     C <- L                     |
|      **4e**      |     MOV C,M     |    1     |        -        |                   C <- (HL)                    |
|      **4f**      |     MOV C,A     |    1     |        -        |                     C <- A                     |
|      **32**      |     MOV D,B     |    1     |        -        |                     D <- B                     |
|      **33**      |     MOV D,C     |    1     |        -        |                     D <- C                     |
|      **34**      |     MOV D,D     |    1     |        -        |                     D <- D                     |
|      **35**      |     MOV D,E     |    1     |        -        |                     D <- E                     |
|      **36**      |     MOV D,H     |    1     |        -        |                     D <- H                     |
|      **37**      |     MOV D,L     |    1     |        -        |                     D <- L                     |
|      **38**      |     MOV D,M     |    1     |        -        |                   D <- (HL)                    |
|      **39**      |     MOV D,A     |    1     |        -        |                     D <- A                     |
|      **3A**      |     MOV E,B     |    1     |        -        |                     E <- B                     |
|      **3B**      |     MOV E,C     |    1     |        -        |                     E <- C                     |
|      **5a**      |     MOV E,D     |    1     |        -        |                     E <- D                     |
|      **5b**      |     MOV E,E     |    1     |        -        |                     E <- E                     |
|      **5c**      |     MOV E,H     |    1     |        -        |                     E <- H                     |
|      **5d**      |     MOV E,L     |    1     |        -        |                     E <- L                     |
|      **5e**      |     MOV E,M     |    1     |        -        |                   E <- (HL)                    |
|      **5f**      |     MOV E,A     |    1     |        -        |                     E <- A                     |
|      **3C**      |     MOV H,B     |    1     |        -        |                     H <- B                     |
|      **3D**      |     MOV H,C     |    1     |        -        |                     H <- C                     |
|      **3E**      |     MOV H,D     |    1     |        -        |                     H <- D                     |
|      **3F**      |     MOV H,E     |    1     |        -        |                     H <- E                     |
|      **40**      |     MOV H,H     |    1     |        -        |                     H <- H                     |
|      **41**      |     MOV H,L     |    1     |        -        |                     H <- L                     |
|      **42**      |     MOV H,M     |    1     |        -        |                   H <- (HL)                    |
|      **43**      |     MOV H,A     |    1     |        -        |                     H <- A                     |
|      **44**      |     MOV L,B     |    1     |        -        |                     L <- B                     |
|      **45**      |     MOV L,C     |    1     |        -        |                     L <- C                     |
|      **6a**      |     MOV L,D     |    1     |        -        |                     L <- D                     |
|      **6b**      |     MOV L,E     |    1     |        -        |                     L <- E                     |
|      **6c**      |     MOV L,H     |    1     |        -        |                     L <- H                     |
|      **6d**      |     MOV L,L     |    1     |        -        |                     L <- L                     |
|      **6e**      |     MOV L,M     |    1     |        -        |                   L <- (HL)                    |
|      **6f**      |     MOV L,A     |    1     |        -        |                     L <- A                     |
|      **46**      |     MOV M,B     |    1     |        -        |                   (HL) <- B                    |
|      **47**      |     MOV M,C     |    1     |        -        |                   (HL) <- C                    |
|      **48**      |     MOV M,D     |    1     |        -        |                   (HL) <- D                    |
|      **49**      |     MOV M,E     |    1     |        -        |                   (HL) <- E                    |
|      **4A**      |     MOV M,H     |    1     |        -        |                   (HL) <- H                    |
|      **4B**      |     MOV M,L     |    1     |        -        |                   (HL) <- L                    |
|      **4C**      |       HLT       |    1     |        -        |                    special                     |
|      **4D**      |     MOV M,A     |    1     |        -        |                   (HL) <- A                    |
|      **4E**      |     MOV A,B     |    1     |        -        |                     A <- B                     |
|      **4F**      |     MOV A,C     |    1     |        -        |                     A <- C                     |
|      **7a**      |     MOV A,D     |    1     |        -        |                     A <- D                     |
|      **7b**      |     MOV A,E     |    1     |        -        |                     A <- E                     |
|      **7c**      |     MOV A,H     |    1     |        -        |                     A <- H                     |
|      **7d**      |     MOV A,L     |    1     |        -        |                     A <- L                     |
|      **7e**      |     MOV A,M     |    1     |        -        |                   A <- (HL)                    |
|      **7f**      |     MOV A,A     |    1     |        -        |                     A <- A                     |
|      **50**      |      ADD B      |    1     | Z, S, P, CY, AC |                   A <- A + B                   |
|      **51**      |      ADD C      |    1     | Z, S, P, CY, AC |                   A <- A + C                   |
|      **52**      |      ADD D      |    1     | Z, S, P, CY, AC |                   A <- A + D                   |
|      **53**      |      ADD E      |    1     | Z, S, P, CY, AC |                   A <- A + E                   |
|      **54**      |      ADD H      |    1     | Z, S, P, CY, AC |                   A <- A + H                   |
|      **55**      |      ADD L      |    1     | Z, S, P, CY, AC |                   A <- A + L                   |
|      **56**      |      ADD M      |    1     | Z, S, P, CY, AC |                 A <- A + (HL)                  |
|      **57**      |      ADD A      |    1     | Z, S, P, CY, AC |                   A <- A + A                   |
|      **58**      |      ADC B      |    1     | Z, S, P, CY, AC |                A <- A + B + CY                 |
|      **59**      |      ADC C      |    1     | Z, S, P, CY, AC |                A <- A + C + CY                 |
|      **8a**      |      ADC D      |    1     | Z, S, P, CY, AC |                A <- A + D + CY                 |
|      **8b**      |      ADC E      |    1     | Z, S, P, CY, AC |                A <- A + E + CY                 |
|      **8c**      |      ADC H      |    1     | Z, S, P, CY, AC |                A <- A + H + CY                 |
|      **8d**      |      ADC L      |    1     | Z, S, P, CY, AC |                A <- A + L + CY                 |
|      **8e**      |      ADC M      |    1     | Z, S, P, CY, AC |               A <- A + (HL) + CY               |
|      **8f**      |      ADC A      |    1     | Z, S, P, CY, AC |                A <- A + A + CY                 |
|      **5A**      |      SUB B      |    1     | Z, S, P, CY, AC |                   A <- A - B                   |
|      **5B**      |      SUB C      |    1     | Z, S, P, CY, AC |                   A <- A - C                   |
|      **5C**      |      SUB D      |    1     | Z, S, P, CY, AC |                   A <- A + D                   |
|      **5D**      |      SUB E      |    1     | Z, S, P, CY, AC |                   A <- A - E                   |
|      **5E**      |      SUB H      |    1     | Z, S, P, CY, AC |                   A <- A + H                   |
|      **5F**      |      SUB L      |    1     | Z, S, P, CY, AC |                   A <- A - L                   |
|      **60**      |      SUB M      |    1     | Z, S, P, CY, AC |                 A <- A + (HL)                  |
|      **61**      |      SUB A      |    1     | Z, S, P, CY, AC |                   A <- A - A                   |
|      **62**      |      SBB B      |    1     | Z, S, P, CY, AC |                A <- A - B - CY                 |
|      **63**      |      SBB C      |    1     | Z, S, P, CY, AC |                A <- A - C - CY                 |
|      **9a**      |      SBB D      |    1     | Z, S, P, CY, AC |                A <- A - D - CY                 |
|      **9b**      |      SBB E      |    1     | Z, S, P, CY, AC |                A <- A - E - CY                 |
|      **9c**      |      SBB H      |    1     | Z, S, P, CY, AC |                A <- A - H - CY                 |
|      **9d**      |      SBB L      |    1     | Z, S, P, CY, AC |                A <- A - L - CY                 |
|      **9e**      |      SBB M      |    1     | Z, S, P, CY, AC |               A <- A - (HL) - CY               |
|      **9f**      |      SBB A      |    1     | Z, S, P, CY, AC |                A <- A - A - CY                 |
|      **a0**      |      ANA B      |    1     | Z, S, P, CY, AC |                   A <- A & B                   |
|      **a1**      |      ANA C      |    1     | Z, S, P, CY, AC |                   A <- A & C                   |
|      **a2**      |      ANA D      |    1     | Z, S, P, CY, AC |                   A <- A & D                   |
|      **a3**      |      ANA E      |    1     | Z, S, P, CY, AC |                   A <- A & E                   |
|      **a4**      |      ANA H      |    1     | Z, S, P, CY, AC |                   A <- A & H                   |
|      **a5**      |      ANA L      |    1     | Z, S, P, CY, AC |                   A <- A & L                   |
|      **a6**      |      ANA M      |    1     | Z, S, P, CY, AC |                 A <- A & (HL)                  |
|      **a7**      |      ANA A      |    1     | Z, S, P, CY, AC |                   A <- A & A                   |
|      **a8**      |      XRA B      |    1     | Z, S, P, CY, AC |                   A <- A ^ B                   |
|      **a9**      |      XRA C      |    1     | Z, S, P, CY, AC |                   A <- A ^ C                   |
|      **aa**      |      XRA D      |    1     | Z, S, P, CY, AC |                   A <- A ^ D                   |
|      **ab**      |      XRA E      |    1     | Z, S, P, CY, AC |                   A <- A ^ E                   |
|      **ac**      |      XRA H      |    1     | Z, S, P, CY, AC |                   A <- A ^ H                   |
|      **ad**      |      XRA L      |    1     | Z, S, P, CY, AC |                   A <- A ^ L                   |
|      **ae**      |      XRA M      |    1     | Z, S, P, CY, AC |                 A <- A ^ (HL)                  |
|      **af**      |      XRA A      |    1     | Z, S, P, CY, AC |                   A <- A ^ A                   |
|      **b0**      |      ORA B      |    1     | Z, S, P, CY, AC |                  A <- A or B                   |
|      **b1**      |      ORA C      |    1     | Z, S, P, CY, AC |                  A <- A or C                   |
|      **b2**      |      ORA D      |    1     | Z, S, P, CY, AC |                  A <- A or D                   |
|      **b3**      |      ORA E      |    1     | Z, S, P, CY, AC |                  A <- A or E                   |
|      **b4**      |      ORA H      |    1     | Z, S, P, CY, AC |                  A <- A or H                   |
|      **b5**      |      ORA L      |    1     | Z, S, P, CY, AC |                  A <- A or L                   |
|      **b6**      |      ORA M      |    1     | Z, S, P, CY, AC |                 A <- A or (HL)                 |
|      **b7**      |      ORA A      |    1     | Z, S, P, CY, AC |                  A <- A or A                   |
|      **b8**      |      CMP B      |    1     | Z, S, P, CY, AC |                      A -B                      |
|      **b9**      |      CMP C      |    1     | Z, S, P, CY, AC |                      A -C                      |
|      **ba**      |      CMP D      |    1     | Z, S, P, CY, AC |                      A -D                      |
|      **bb**      |      CMP E      |    1     | Z, S, P, CY, AC |                      A -E                      |
|      **bc**      |      CMP H      |    1     | Z, S, P, CY, AC |                      A -H                      |
|      **bd**      |      CMP L      |    1     | Z, S, P, CY, AC |                      A -L                      |
|      **be**      |      CMP M      |    1     | Z, S, P, CY, AC |                    A - (HL)                    |
|      **bf**      |      CMP A      |    1     | Z, S, P, CY, AC |                      A -A                      |
|      **c0**      |       RNZ       |    1     |        -        |                   if NZ, RET                   |
|      **c1**      |      POP B      |    1     |        -        |       C <- (sp); B <- (sp+1); sp <- sp+2       |
|      **c2**      |     JNZ adr     |    3     |        -        |                if NZ, PC <- adr                |
|      **c3**      |     JMP adr     |    3     |        -        |                   PC <= adr                    |
|      **c4**      |     CNZ adr     |    3     |        -        |                if NZ, CALL adr                 |
|      **c5**      |     PUSH B      |    1     |        -        |       (sp-2)<-C; (sp-1)<-B; sp <- sp - 2       |
|      **c6**      |     ADI D8      |    2     | Z, S, P, CY, AC |                 A <- A + byte                  |
|      **c7**      |      RST 0      |    1     |        -        |                    CALL $0                     |
|      **c8**      |       RZ        |    1     |        -        |                   if Z, RET                    |
|      **c9**      |       RET       |    1     |        -        |    PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2    |
|      **ca**      |     JZ adr      |    3     |        -        |                if Z, PC <- adr                 |
|      **cb**      |        -        |    -     |        -        |                       -                        |
|      **cc**      |     CZ adr      |    3     |        -        |                 if Z, CALL adr                 |
|      **cd**      |    CALL adr     |    3     |        -        |  (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr   |
|      **ce**      |     ACI D8      |    2     | Z, S, P, CY, AC |               A <- A + data + CY               |
|      **cf**      |      RST 1      |    1     |        -        |                    CALL $8                     |
|      **d0**      |       RNC       |    1     |        -        |                  if NCY, RET                   |
|      **d1**      |      POP D      |    1     |        -        |       E <- (sp); D <- (sp+1); sp <- sp+2       |
|      **d2**      |     JNC adr     |    3     |        -        |                if NCY, PC<-adr                 |
|      **d3**      |     OUT D8      |    2     |        -        |                    special                     |
|      **d4**      |     CNC adr     |    3     |        -        |                if NCY, CALL adr                |
|      **d5**      |     PUSH D      |    1     |        -        |       (sp-2)<-E; (sp-1)<-D; sp <- sp - 2       |
|      **d6**      |     SUI D8      |    2     | Z, S, P, CY, AC |                 A <- A - data                  |
|      **d7**      |      RST 2      |    1     |        -        |                    CALL $10                    |
|      **d8**      |       RC        |    1     |        -        |                   if CY, RET                   |
|      **d9**      |        -        |    -     |        -        |                       -                        |
|      **da**      |     JC adr      |    3     |        -        |                 if CY, PC<-adr                 |
|      **db**      |      IN D8      |    2     |        -        |                    special                     |
|      **dc**      |     CC adr      |    3     |        -        |                if CY, CALL adr                 |
|      **dd**      |        -        |    -     |        -        |                       -                        |
|      **de**      |     SBI D8      |    2     | Z, S, P, CY, AC |               A <- A - data - CY               |
|      **df**      |      RST 3      |    1     |        -        |                    CALL $18                    |
|      **e0**      |       RPO       |    1     |        -        |                   if PO, RET                   |
|      **e1**      |      POP H      |    1     |        -        |       L <- (sp); H <- (sp+1); sp <- sp+2       |
|      **e2**      |     JPO adr     |    3     |        -        |                if PO, PC <- adr                |
|      **e3**      |      XTHL       |    1     |        -        |               L  (SP); H  (SP+1)               |
|      **e4**      |     CPO adr     |    3     |        -        |                if PO, CALL adr                 |
|      **e5**      |     PUSH H      |    1     |        -        |       (sp-2)<-L; (sp-1)<-H; sp <- sp - 2       |
|      **e6**      |     ANI D8      |    2     | Z, S, P, CY, AC |                 A <- A & data                  |
|      **e7**      |      RST 4      |    1     |        -        |                    CALL $20                    |
|      **e8**      |       RPE       |    1     |        -        |                   if PE, RET                   |
|      **e9**      |      PCHL       |    1     |        -        |             PC.hi <- H; PC.lo <- L             |
|      **ea**      |     JPE adr     |    3     |        -        |                if PE, PC <- adr                |
|      **eb**      |      XCHG       |    1     |        -        |                   H  D; L  E                   |
|      **ec**      |     CPE adr     |    3     |        -        |                if PE, CALL adr                 |
|      **ed**      |        -        |    -     |        -        |                       -                        |
|      **ee**      |     XRI D8      |    2     | Z, S, P, CY, AC |                 A <- A ^ data                  |
|      **ef**      |      RST 5      |    1     |        -        |                    CALL $28                    |
|      **f0**      |       RP        |    1     |        -        |                   if P, RET                    |
|      **f1**      |     POP PSW     |    1     |        -        |     flags <- (sp); A <- (sp+1); sp <- sp+2     |
|      **f2**      |     JP adr      |    3     |        -        |                if P=1 PC <- adr                |
|      **f3**      |       DI        |    1     |        -        |                    special                     |
|      **f4**      |     CP adr      |    3     |        -        |                if P, PC <- adr                 |
|      **f5**      |    PUSH PSW     |    1     |        -        |     (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2     |
|      **f6**      |     ORI D8      |    2     | Z, S, P, CY, AC |                 A <- A or data                 |
|      **f7**      |      RST 6      |    1     |        -        |                    CALL $30                    |
|      **f8**      |       RM        |    1     |        -        |                   if M, RET                    |
|      **f9**      |      SPHL       |    1     |        -        |                     SP=HL                      |
|      **fa**      |     JM adr      |    3     |        -        |                if M, PC <- adr                 |
|      **fb**      |       EI        |    1     |        -        |                    special                     |
|      **fc**      |     CM adr      |    3     |        -        |                 if M, CALL adr                 |
|      **fd**      |        -        |    -     |        -        |                       -                        |
|      **fe**      |     CPI D8      |    2     | Z, S, P, CY, AC |                    A - data                    |
|      **ff**      |      RST 7      |    1     |        -        |                    CALL $38                    |
