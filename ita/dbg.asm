
target/dev/x86_64-unknown-linux-gnu/bin/ita-d5781e7b9810bf27:	file format elf64-x86-64

Disassembly of section .text:

0000000000024933 <sha256.sha256Compress>:
   24933: 55                           	pushq	%rbp
   24934: 48 89 e5                     	movq	%rsp, %rbp
   24937: 48 81 ec 60 02 00 00         	subq	$0x260, %rsp            # imm = 0x260
   2493e: 48 89 9c 24 30 02 00 00      	movq	%rbx, 0x230(%rsp)
   24946: 4c 89 a4 24 38 02 00 00      	movq	%r12, 0x238(%rsp)
   2494e: 4c 89 ac 24 40 02 00 00      	movq	%r13, 0x240(%rsp)
   24956: 4c 89 b4 24 48 02 00 00      	movq	%r14, 0x248(%rsp)
   2495e: 4c 89 bc 24 50 02 00 00      	movq	%r15, 0x250(%rsp)
   24966: 48 89 3c 24                  	movq	%rdi, (%rsp)
   2496a: 48 33 f6                     	xorq	%rsi, %rsi
   2496d: 48 8b 05 34 40 05 00         	movq	0x54034(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24974: ff d0                        	callq	*%rax
   24976: 48 89 44 24 18               	movq	%rax, 0x18(%rsp)
   2497b: be 04 00 00 00               	movl	$0x4, %esi
   24980: 48 8b 05 21 40 05 00         	movq	0x54021(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24987: 48 8b 3c 24                  	movq	(%rsp), %rdi
   2498b: ff d0                        	callq	*%rax
   2498d: 48 89 44 24 10               	movq	%rax, 0x10(%rsp)
   24992: be 08 00 00 00               	movl	$0x8, %esi
   24997: 48 8b 05 0a 40 05 00         	movq	0x5400a(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   2499e: 48 8b 3c 24                  	movq	(%rsp), %rdi
   249a2: ff d0                        	callq	*%rax
   249a4: 48 89 84 24 20 02 00 00      	movq	%rax, 0x220(%rsp)
   249ac: be 0c 00 00 00               	movl	$0xc, %esi
   249b1: 48 8b 05 f0 3f 05 00         	movq	0x53ff0(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   249b8: 48 8b 3c 24                  	movq	(%rsp), %rdi
   249bc: ff d0                        	callq	*%rax
   249be: 48 89 84 24 18 02 00 00      	movq	%rax, 0x218(%rsp)
   249c6: be 10 00 00 00               	movl	$0x10, %esi
   249cb: 48 8b 05 d6 3f 05 00         	movq	0x53fd6(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   249d2: 48 8b 3c 24                  	movq	(%rsp), %rdi
   249d6: ff d0                        	callq	*%rax
   249d8: 48 89 84 24 10 02 00 00      	movq	%rax, 0x210(%rsp)
   249e0: be 14 00 00 00               	movl	$0x14, %esi
   249e5: 48 8b 05 bc 3f 05 00         	movq	0x53fbc(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   249ec: 48 8b 3c 24                  	movq	(%rsp), %rdi
   249f0: ff d0                        	callq	*%rax
   249f2: 48 89 84 24 08 02 00 00      	movq	%rax, 0x208(%rsp)
   249fa: be 18 00 00 00               	movl	$0x18, %esi
   249ff: 48 8b 05 a2 3f 05 00         	movq	0x53fa2(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24a06: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24a0a: ff d0                        	callq	*%rax
   24a0c: 48 89 84 24 00 02 00 00      	movq	%rax, 0x200(%rsp)
   24a14: be 1c 00 00 00               	movl	$0x1c, %esi
   24a19: 48 8b 05 88 3f 05 00         	movq	0x53f88(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24a20: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24a24: ff d0                        	callq	*%rax
   24a26: 48 89 84 24 f8 01 00 00      	movq	%rax, 0x1f8(%rsp)
   24a2e: be 20 00 00 00               	movl	$0x20, %esi
   24a33: 48 8b 05 6e 3f 05 00         	movq	0x53f6e(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24a3a: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24a3e: ff d0                        	callq	*%rax
   24a40: 48 89 84 24 f0 01 00 00      	movq	%rax, 0x1f0(%rsp)
   24a48: be 24 00 00 00               	movl	$0x24, %esi
   24a4d: 48 8b 05 54 3f 05 00         	movq	0x53f54(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24a54: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24a58: ff d0                        	callq	*%rax
   24a5a: 48 89 84 24 e8 01 00 00      	movq	%rax, 0x1e8(%rsp)
   24a62: be 28 00 00 00               	movl	$0x28, %esi
   24a67: 48 8b 05 3a 3f 05 00         	movq	0x53f3a(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24a6e: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24a72: ff d0                        	callq	*%rax
   24a74: 48 89 84 24 e0 01 00 00      	movq	%rax, 0x1e0(%rsp)
   24a7c: be 2c 00 00 00               	movl	$0x2c, %esi
   24a81: 48 8b 05 20 3f 05 00         	movq	0x53f20(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24a88: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24a8c: ff d0                        	callq	*%rax
   24a8e: 48 89 84 24 d8 01 00 00      	movq	%rax, 0x1d8(%rsp)
   24a96: be 30 00 00 00               	movl	$0x30, %esi
   24a9b: 48 8b 05 06 3f 05 00         	movq	0x53f06(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24aa2: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24aa6: ff d0                        	callq	*%rax
   24aa8: 48 89 84 24 d0 01 00 00      	movq	%rax, 0x1d0(%rsp)
   24ab0: be 34 00 00 00               	movl	$0x34, %esi
   24ab5: 48 8b 05 ec 3e 05 00         	movq	0x53eec(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24abc: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24ac0: ff d0                        	callq	*%rax
   24ac2: 48 89 84 24 c8 01 00 00      	movq	%rax, 0x1c8(%rsp)
   24aca: be 38 00 00 00               	movl	$0x38, %esi
   24acf: 48 8b 05 d2 3e 05 00         	movq	0x53ed2(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24ad6: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24ada: ff d0                        	callq	*%rax
   24adc: 48 89 84 24 c0 01 00 00      	movq	%rax, 0x1c0(%rsp)
   24ae4: be 3c 00 00 00               	movl	$0x3c, %esi
   24ae9: 48 8b 05 b8 3e 05 00         	movq	0x53eb8(%rip), %rax     # 0x789a8 <sha256.beU32Buf$got>
   24af0: 48 8b 3c 24                  	movq	(%rsp), %rdi
   24af4: ff d0                        	callq	*%rax
   24af6: 48 89 84 24 b8 01 00 00      	movq	%rax, 0x1b8(%rsp)
   24afe: 48 8b 05 7b 3e 05 00         	movq	0x53e7b(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24b05: 48 8b bc 24 c0 01 00 00      	movq	0x1c0(%rsp), %rdi
   24b0d: ff d0                        	callq	*%rax
   24b0f: 48 8b 8c 24 e8 01 00 00      	movq	0x1e8(%rsp), %rcx
   24b17: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24b1a: 48 8b 0d 57 3e 05 00         	movq	0x53e57(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24b21: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   24b26: ff d1                        	callq	*%rcx
   24b28: 03 c3                        	addl	%ebx, %eax
   24b2a: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   24b2f: 8d 0c 10                     	leal	(%rax,%rdx), %ecx
   24b32: 48 89 8c 24 b0 01 00 00      	movq	%rcx, 0x1b0(%rsp)
   24b3a: 48 8b 05 3f 3e 05 00         	movq	0x53e3f(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24b41: 48 8b bc 24 b8 01 00 00      	movq	0x1b8(%rsp), %rdi
   24b49: ff d0                        	callq	*%rax
   24b4b: 48 8b 8c 24 e0 01 00 00      	movq	0x1e0(%rsp), %rcx
   24b53: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24b56: 48 8b 0d 1b 3e 05 00         	movq	0x53e1b(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24b5d: 48 8b bc 24 20 02 00 00      	movq	0x220(%rsp), %rdi
   24b65: ff d1                        	callq	*%rcx
   24b67: 03 c3                        	addl	%ebx, %eax
   24b69: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   24b6e: 03 c8                        	addl	%eax, %ecx
   24b70: 48 89 8c 24 a8 01 00 00      	movq	%rcx, 0x1a8(%rsp)
   24b78: 48 8b 05 01 3e 05 00         	movq	0x53e01(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24b7f: 48 8b bc 24 b0 01 00 00      	movq	0x1b0(%rsp), %rdi
   24b87: ff d0                        	callq	*%rax
   24b89: 48 8b 8c 24 d8 01 00 00      	movq	0x1d8(%rsp), %rcx
   24b91: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24b94: 48 8b 0d dd 3d 05 00         	movq	0x53ddd(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24b9b: 48 8b bc 24 18 02 00 00      	movq	0x218(%rsp), %rdi
   24ba3: ff d1                        	callq	*%rcx
   24ba5: 03 c3                        	addl	%ebx, %eax
   24ba7: 48 8b 8c 24 20 02 00 00      	movq	0x220(%rsp), %rcx
   24baf: 03 c8                        	addl	%eax, %ecx
   24bb1: 48 89 8c 24 a0 01 00 00      	movq	%rcx, 0x1a0(%rsp)
   24bb9: 48 8b 05 c0 3d 05 00         	movq	0x53dc0(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24bc0: 48 8b bc 24 a8 01 00 00      	movq	0x1a8(%rsp), %rdi
   24bc8: ff d0                        	callq	*%rax
   24bca: 48 8b 8c 24 d0 01 00 00      	movq	0x1d0(%rsp), %rcx
   24bd2: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24bd5: 48 8b 0d 9c 3d 05 00         	movq	0x53d9c(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24bdc: 48 8b bc 24 10 02 00 00      	movq	0x210(%rsp), %rdi
   24be4: ff d1                        	callq	*%rcx
   24be6: 03 c3                        	addl	%ebx, %eax
   24be8: 48 8b 8c 24 18 02 00 00      	movq	0x218(%rsp), %rcx
   24bf0: 03 c8                        	addl	%eax, %ecx
   24bf2: 48 89 8c 24 98 01 00 00      	movq	%rcx, 0x198(%rsp)
   24bfa: 48 8b 05 7f 3d 05 00         	movq	0x53d7f(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24c01: 48 8b bc 24 a0 01 00 00      	movq	0x1a0(%rsp), %rdi
   24c09: ff d0                        	callq	*%rax
   24c0b: 48 8b 8c 24 c8 01 00 00      	movq	0x1c8(%rsp), %rcx
   24c13: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24c16: 48 8b 0d 5b 3d 05 00         	movq	0x53d5b(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24c1d: 48 8b bc 24 08 02 00 00      	movq	0x208(%rsp), %rdi
   24c25: ff d1                        	callq	*%rcx
   24c27: 03 c3                        	addl	%ebx, %eax
   24c29: 48 8b 8c 24 10 02 00 00      	movq	0x210(%rsp), %rcx
   24c31: 03 c8                        	addl	%eax, %ecx
   24c33: 48 89 8c 24 90 01 00 00      	movq	%rcx, 0x190(%rsp)
   24c3b: 48 8b 05 3e 3d 05 00         	movq	0x53d3e(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24c42: 48 8b bc 24 98 01 00 00      	movq	0x198(%rsp), %rdi
   24c4a: ff d0                        	callq	*%rax
   24c4c: 48 8b 8c 24 c0 01 00 00      	movq	0x1c0(%rsp), %rcx
   24c54: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24c57: 48 8b 0d 1a 3d 05 00         	movq	0x53d1a(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24c5e: 48 8b bc 24 00 02 00 00      	movq	0x200(%rsp), %rdi
   24c66: ff d1                        	callq	*%rcx
   24c68: 03 c3                        	addl	%ebx, %eax
   24c6a: 48 8b 94 24 08 02 00 00      	movq	0x208(%rsp), %rdx
   24c72: 8d 0c 10                     	leal	(%rax,%rdx), %ecx
   24c75: 48 89 8c 24 88 01 00 00      	movq	%rcx, 0x188(%rsp)
   24c7d: 48 8b 05 fc 3c 05 00         	movq	0x53cfc(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24c84: 48 8b bc 24 90 01 00 00      	movq	0x190(%rsp), %rdi
   24c8c: ff d0                        	callq	*%rax
   24c8e: 48 8b b4 24 b8 01 00 00      	movq	0x1b8(%rsp), %rsi
   24c96: 8d 1c 30                     	leal	(%rax,%rsi), %ebx
   24c99: 48 8b 0d d8 3c 05 00         	movq	0x53cd8(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24ca0: 48 8b bc 24 f8 01 00 00      	movq	0x1f8(%rsp), %rdi
   24ca8: ff d1                        	callq	*%rcx
   24caa: 03 c3                        	addl	%ebx, %eax
   24cac: 48 8b b4 24 00 02 00 00      	movq	0x200(%rsp), %rsi
   24cb4: 8d 0c 30                     	leal	(%rax,%rsi), %ecx
   24cb7: 48 89 8c 24 80 01 00 00      	movq	%rcx, 0x180(%rsp)
   24cbf: 48 8b 05 ba 3c 05 00         	movq	0x53cba(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24cc6: 48 8b bc 24 88 01 00 00      	movq	0x188(%rsp), %rdi
   24cce: ff d0                        	callq	*%rax
   24cd0: 48 8b 8c 24 b0 01 00 00      	movq	0x1b0(%rsp), %rcx
   24cd8: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24cdb: 48 8b 0d 96 3c 05 00         	movq	0x53c96(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24ce2: 48 8b bc 24 f0 01 00 00      	movq	0x1f0(%rsp), %rdi
   24cea: ff d1                        	callq	*%rcx
   24cec: 03 c3                        	addl	%ebx, %eax
   24cee: 4c 8b 84 24 f8 01 00 00      	movq	0x1f8(%rsp), %r8
   24cf6: 42 8d 0c 00                  	leal	(%rax,%r8), %ecx
   24cfa: 48 89 8c 24 78 01 00 00      	movq	%rcx, 0x178(%rsp)
   24d02: 48 8b 05 77 3c 05 00         	movq	0x53c77(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24d09: 48 8b bc 24 80 01 00 00      	movq	0x180(%rsp), %rdi
   24d11: ff d0                        	callq	*%rax
   24d13: 48 8b 8c 24 a8 01 00 00      	movq	0x1a8(%rsp), %rcx
   24d1b: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24d1e: 48 8b 0d 53 3c 05 00         	movq	0x53c53(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24d25: 48 8b bc 24 e8 01 00 00      	movq	0x1e8(%rsp), %rdi
   24d2d: ff d1                        	callq	*%rcx
   24d2f: 03 c3                        	addl	%ebx, %eax
   24d31: 48 8b 8c 24 f0 01 00 00      	movq	0x1f0(%rsp), %rcx
   24d39: 03 c8                        	addl	%eax, %ecx
   24d3b: 48 89 8c 24 70 01 00 00      	movq	%rcx, 0x170(%rsp)
   24d43: 48 8b 05 36 3c 05 00         	movq	0x53c36(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24d4a: 48 8b bc 24 78 01 00 00      	movq	0x178(%rsp), %rdi
   24d52: ff d0                        	callq	*%rax
   24d54: 48 8b 8c 24 a0 01 00 00      	movq	0x1a0(%rsp), %rcx
   24d5c: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24d5f: 48 8b 0d 12 3c 05 00         	movq	0x53c12(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24d66: 48 8b bc 24 e0 01 00 00      	movq	0x1e0(%rsp), %rdi
   24d6e: ff d1                        	callq	*%rcx
   24d70: 03 c3                        	addl	%ebx, %eax
   24d72: 48 8b 8c 24 e8 01 00 00      	movq	0x1e8(%rsp), %rcx
   24d7a: 03 c8                        	addl	%eax, %ecx
   24d7c: 48 89 8c 24 68 01 00 00      	movq	%rcx, 0x168(%rsp)
   24d84: 48 8b 05 f5 3b 05 00         	movq	0x53bf5(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24d8b: 48 8b bc 24 70 01 00 00      	movq	0x170(%rsp), %rdi
   24d93: ff d0                        	callq	*%rax
   24d95: 48 8b 8c 24 98 01 00 00      	movq	0x198(%rsp), %rcx
   24d9d: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24da0: 48 8b 0d d1 3b 05 00         	movq	0x53bd1(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24da7: 48 8b bc 24 d8 01 00 00      	movq	0x1d8(%rsp), %rdi
   24daf: ff d1                        	callq	*%rcx
   24db1: 03 c3                        	addl	%ebx, %eax
   24db3: 48 8b 8c 24 e0 01 00 00      	movq	0x1e0(%rsp), %rcx
   24dbb: 03 c8                        	addl	%eax, %ecx
   24dbd: 48 89 8c 24 60 01 00 00      	movq	%rcx, 0x160(%rsp)
   24dc5: 48 8b 05 b4 3b 05 00         	movq	0x53bb4(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24dcc: 48 8b bc 24 68 01 00 00      	movq	0x168(%rsp), %rdi
   24dd4: ff d0                        	callq	*%rax
   24dd6: 48 8b 8c 24 90 01 00 00      	movq	0x190(%rsp), %rcx
   24dde: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24de1: 48 8b 0d 90 3b 05 00         	movq	0x53b90(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24de8: 48 8b bc 24 d0 01 00 00      	movq	0x1d0(%rsp), %rdi
   24df0: ff d1                        	callq	*%rcx
   24df2: 03 c3                        	addl	%ebx, %eax
   24df4: 48 8b 8c 24 d8 01 00 00      	movq	0x1d8(%rsp), %rcx
   24dfc: 03 c8                        	addl	%eax, %ecx
   24dfe: 48 89 8c 24 58 01 00 00      	movq	%rcx, 0x158(%rsp)
   24e06: 48 8b 05 73 3b 05 00         	movq	0x53b73(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24e0d: 48 8b bc 24 60 01 00 00      	movq	0x160(%rsp), %rdi
   24e15: ff d0                        	callq	*%rax
   24e17: 48 8b 8c 24 88 01 00 00      	movq	0x188(%rsp), %rcx
   24e1f: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24e22: 48 8b 0d 4f 3b 05 00         	movq	0x53b4f(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24e29: 48 8b bc 24 c8 01 00 00      	movq	0x1c8(%rsp), %rdi
   24e31: ff d1                        	callq	*%rcx
   24e33: 03 c3                        	addl	%ebx, %eax
   24e35: 48 8b 8c 24 d0 01 00 00      	movq	0x1d0(%rsp), %rcx
   24e3d: 03 c8                        	addl	%eax, %ecx
   24e3f: 48 89 8c 24 50 01 00 00      	movq	%rcx, 0x150(%rsp)
   24e47: 48 8b 05 32 3b 05 00         	movq	0x53b32(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24e4e: 48 8b bc 24 58 01 00 00      	movq	0x158(%rsp), %rdi
   24e56: ff d0                        	callq	*%rax
   24e58: 48 8b 8c 24 80 01 00 00      	movq	0x180(%rsp), %rcx
   24e60: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24e63: 48 8b 0d 0e 3b 05 00         	movq	0x53b0e(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24e6a: 48 8b bc 24 c0 01 00 00      	movq	0x1c0(%rsp), %rdi
   24e72: ff d1                        	callq	*%rcx
   24e74: 03 c3                        	addl	%ebx, %eax
   24e76: 48 8b 8c 24 c8 01 00 00      	movq	0x1c8(%rsp), %rcx
   24e7e: 03 c8                        	addl	%eax, %ecx
   24e80: 48 89 8c 24 48 01 00 00      	movq	%rcx, 0x148(%rsp)
   24e88: 48 8b 05 f1 3a 05 00         	movq	0x53af1(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24e8f: 48 8b bc 24 50 01 00 00      	movq	0x150(%rsp), %rdi
   24e97: ff d0                        	callq	*%rax
   24e99: 48 8b 8c 24 78 01 00 00      	movq	0x178(%rsp), %rcx
   24ea1: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24ea4: 48 8b 0d cd 3a 05 00         	movq	0x53acd(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24eab: 48 8b bc 24 b8 01 00 00      	movq	0x1b8(%rsp), %rdi
   24eb3: ff d1                        	callq	*%rcx
   24eb5: 03 c3                        	addl	%ebx, %eax
   24eb7: 48 8b 8c 24 c0 01 00 00      	movq	0x1c0(%rsp), %rcx
   24ebf: 03 c8                        	addl	%eax, %ecx
   24ec1: 48 89 8c 24 40 01 00 00      	movq	%rcx, 0x140(%rsp)
   24ec9: 48 8b 05 b0 3a 05 00         	movq	0x53ab0(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24ed0: 48 8b bc 24 48 01 00 00      	movq	0x148(%rsp), %rdi
   24ed8: ff d0                        	callq	*%rax
   24eda: 48 8b 8c 24 70 01 00 00      	movq	0x170(%rsp), %rcx
   24ee2: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24ee5: 48 8b 0d 8c 3a 05 00         	movq	0x53a8c(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24eec: 48 8b bc 24 b0 01 00 00      	movq	0x1b0(%rsp), %rdi
   24ef4: ff d1                        	callq	*%rcx
   24ef6: 03 c3                        	addl	%ebx, %eax
   24ef8: 48 8b b4 24 b8 01 00 00      	movq	0x1b8(%rsp), %rsi
   24f00: 8d 0c 30                     	leal	(%rax,%rsi), %ecx
   24f03: 48 89 8c 24 38 01 00 00      	movq	%rcx, 0x138(%rsp)
   24f0b: 48 8b 05 6e 3a 05 00         	movq	0x53a6e(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24f12: 48 8b bc 24 40 01 00 00      	movq	0x140(%rsp), %rdi
   24f1a: ff d0                        	callq	*%rax
   24f1c: 48 8b 8c 24 68 01 00 00      	movq	0x168(%rsp), %rcx
   24f24: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24f27: 48 8b 0d 4a 3a 05 00         	movq	0x53a4a(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24f2e: 48 8b bc 24 a8 01 00 00      	movq	0x1a8(%rsp), %rdi
   24f36: ff d1                        	callq	*%rcx
   24f38: 03 c3                        	addl	%ebx, %eax
   24f3a: 48 8b 8c 24 b0 01 00 00      	movq	0x1b0(%rsp), %rcx
   24f42: 03 c8                        	addl	%eax, %ecx
   24f44: 48 89 8c 24 30 01 00 00      	movq	%rcx, 0x130(%rsp)
   24f4c: 48 8b 05 2d 3a 05 00         	movq	0x53a2d(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24f53: 48 8b bc 24 38 01 00 00      	movq	0x138(%rsp), %rdi
   24f5b: ff d0                        	callq	*%rax
   24f5d: 48 8b 8c 24 60 01 00 00      	movq	0x160(%rsp), %rcx
   24f65: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24f68: 48 8b 0d 09 3a 05 00         	movq	0x53a09(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24f6f: 48 8b bc 24 a0 01 00 00      	movq	0x1a0(%rsp), %rdi
   24f77: ff d1                        	callq	*%rcx
   24f79: 03 c3                        	addl	%ebx, %eax
   24f7b: 48 8b 8c 24 a8 01 00 00      	movq	0x1a8(%rsp), %rcx
   24f83: 03 c8                        	addl	%eax, %ecx
   24f85: 48 89 8c 24 28 01 00 00      	movq	%rcx, 0x128(%rsp)
   24f8d: 48 8b 05 ec 39 05 00         	movq	0x539ec(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24f94: 48 8b bc 24 30 01 00 00      	movq	0x130(%rsp), %rdi
   24f9c: ff d0                        	callq	*%rax
   24f9e: 48 8b 8c 24 58 01 00 00      	movq	0x158(%rsp), %rcx
   24fa6: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24fa9: 48 8b 0d c8 39 05 00         	movq	0x539c8(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24fb0: 48 8b bc 24 98 01 00 00      	movq	0x198(%rsp), %rdi
   24fb8: ff d1                        	callq	*%rcx
   24fba: 03 c3                        	addl	%ebx, %eax
   24fbc: 48 8b 8c 24 a0 01 00 00      	movq	0x1a0(%rsp), %rcx
   24fc4: 03 c8                        	addl	%eax, %ecx
   24fc6: 48 89 8c 24 20 01 00 00      	movq	%rcx, 0x120(%rsp)
   24fce: 48 8b 05 ab 39 05 00         	movq	0x539ab(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   24fd5: 48 8b bc 24 28 01 00 00      	movq	0x128(%rsp), %rdi
   24fdd: ff d0                        	callq	*%rax
   24fdf: 48 8b 8c 24 50 01 00 00      	movq	0x150(%rsp), %rcx
   24fe7: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   24fea: 48 8b 0d 87 39 05 00         	movq	0x53987(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   24ff1: 48 8b bc 24 90 01 00 00      	movq	0x190(%rsp), %rdi
   24ff9: ff d1                        	callq	*%rcx
   24ffb: 03 c3                        	addl	%ebx, %eax
   24ffd: 48 8b 8c 24 98 01 00 00      	movq	0x198(%rsp), %rcx
   25005: 03 c8                        	addl	%eax, %ecx
   25007: 48 89 8c 24 18 01 00 00      	movq	%rcx, 0x118(%rsp)
   2500f: 48 8b 05 6a 39 05 00         	movq	0x5396a(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25016: 48 8b bc 24 20 01 00 00      	movq	0x120(%rsp), %rdi
   2501e: ff d0                        	callq	*%rax
   25020: 48 8b 8c 24 48 01 00 00      	movq	0x148(%rsp), %rcx
   25028: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   2502b: 48 8b 0d 46 39 05 00         	movq	0x53946(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25032: 48 8b bc 24 88 01 00 00      	movq	0x188(%rsp), %rdi
   2503a: ff d1                        	callq	*%rcx
   2503c: 03 c3                        	addl	%ebx, %eax
   2503e: 48 8b 8c 24 90 01 00 00      	movq	0x190(%rsp), %rcx
   25046: 03 c8                        	addl	%eax, %ecx
   25048: 48 89 8c 24 10 01 00 00      	movq	%rcx, 0x110(%rsp)
   25050: 48 8b 05 29 39 05 00         	movq	0x53929(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25057: 48 8b bc 24 18 01 00 00      	movq	0x118(%rsp), %rdi
   2505f: ff d0                        	callq	*%rax
   25061: 48 8b 8c 24 40 01 00 00      	movq	0x140(%rsp), %rcx
   25069: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   2506c: 48 8b 0d 05 39 05 00         	movq	0x53905(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25073: 48 8b bc 24 80 01 00 00      	movq	0x180(%rsp), %rdi
   2507b: ff d1                        	callq	*%rcx
   2507d: 03 c3                        	addl	%ebx, %eax
   2507f: 48 8b 8c 24 88 01 00 00      	movq	0x188(%rsp), %rcx
   25087: 03 c8                        	addl	%eax, %ecx
   25089: 48 89 8c 24 08 01 00 00      	movq	%rcx, 0x108(%rsp)
   25091: 48 8b 05 e8 38 05 00         	movq	0x538e8(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25098: 48 8b bc 24 10 01 00 00      	movq	0x110(%rsp), %rdi
   250a0: ff d0                        	callq	*%rax
   250a2: 48 8b 8c 24 38 01 00 00      	movq	0x138(%rsp), %rcx
   250aa: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   250ad: 48 8b 0d c4 38 05 00         	movq	0x538c4(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   250b4: 48 8b bc 24 78 01 00 00      	movq	0x178(%rsp), %rdi
   250bc: ff d1                        	callq	*%rcx
   250be: 03 c3                        	addl	%ebx, %eax
   250c0: 48 8b 8c 24 80 01 00 00      	movq	0x180(%rsp), %rcx
   250c8: 03 c8                        	addl	%eax, %ecx
   250ca: 48 89 8c 24 00 01 00 00      	movq	%rcx, 0x100(%rsp)
   250d2: 48 8b 05 a7 38 05 00         	movq	0x538a7(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   250d9: 48 8b bc 24 08 01 00 00      	movq	0x108(%rsp), %rdi
   250e1: ff d0                        	callq	*%rax
   250e3: 48 8b 8c 24 30 01 00 00      	movq	0x130(%rsp), %rcx
   250eb: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   250ee: 48 8b 0d 83 38 05 00         	movq	0x53883(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   250f5: 48 8b bc 24 70 01 00 00      	movq	0x170(%rsp), %rdi
   250fd: ff d1                        	callq	*%rcx
   250ff: 03 c3                        	addl	%ebx, %eax
   25101: 48 8b 8c 24 78 01 00 00      	movq	0x178(%rsp), %rcx
   25109: 03 c8                        	addl	%eax, %ecx
   2510b: 48 89 8c 24 f8 00 00 00      	movq	%rcx, 0xf8(%rsp)
   25113: 48 8b 05 66 38 05 00         	movq	0x53866(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   2511a: 48 8b bc 24 00 01 00 00      	movq	0x100(%rsp), %rdi
   25122: ff d0                        	callq	*%rax
   25124: 48 8b 8c 24 28 01 00 00      	movq	0x128(%rsp), %rcx
   2512c: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   2512f: 48 8b 0d 42 38 05 00         	movq	0x53842(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25136: 48 8b bc 24 68 01 00 00      	movq	0x168(%rsp), %rdi
   2513e: ff d1                        	callq	*%rcx
   25140: 03 c3                        	addl	%ebx, %eax
   25142: 48 8b 8c 24 70 01 00 00      	movq	0x170(%rsp), %rcx
   2514a: 03 c8                        	addl	%eax, %ecx
   2514c: 48 89 8c 24 f0 00 00 00      	movq	%rcx, 0xf0(%rsp)
   25154: 48 8b 05 25 38 05 00         	movq	0x53825(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   2515b: 48 8b bc 24 f8 00 00 00      	movq	0xf8(%rsp), %rdi
   25163: ff d0                        	callq	*%rax
   25165: 48 8b 8c 24 20 01 00 00      	movq	0x120(%rsp), %rcx
   2516d: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25170: 48 8b 0d 01 38 05 00         	movq	0x53801(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25177: 48 8b bc 24 60 01 00 00      	movq	0x160(%rsp), %rdi
   2517f: ff d1                        	callq	*%rcx
   25181: 03 c3                        	addl	%ebx, %eax
   25183: 48 8b 8c 24 68 01 00 00      	movq	0x168(%rsp), %rcx
   2518b: 03 c8                        	addl	%eax, %ecx
   2518d: 48 89 8c 24 e8 00 00 00      	movq	%rcx, 0xe8(%rsp)
   25195: 48 8b 05 e4 37 05 00         	movq	0x537e4(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   2519c: 48 8b bc 24 f0 00 00 00      	movq	0xf0(%rsp), %rdi
   251a4: ff d0                        	callq	*%rax
   251a6: 48 8b 8c 24 18 01 00 00      	movq	0x118(%rsp), %rcx
   251ae: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   251b1: 48 8b 0d c0 37 05 00         	movq	0x537c0(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   251b8: 48 8b bc 24 58 01 00 00      	movq	0x158(%rsp), %rdi
   251c0: ff d1                        	callq	*%rcx
   251c2: 03 c3                        	addl	%ebx, %eax
   251c4: 48 8b 8c 24 60 01 00 00      	movq	0x160(%rsp), %rcx
   251cc: 03 c8                        	addl	%eax, %ecx
   251ce: 48 89 8c 24 e0 00 00 00      	movq	%rcx, 0xe0(%rsp)
   251d6: 48 8b 05 a3 37 05 00         	movq	0x537a3(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   251dd: 48 8b bc 24 e8 00 00 00      	movq	0xe8(%rsp), %rdi
   251e5: ff d0                        	callq	*%rax
   251e7: 48 8b 8c 24 10 01 00 00      	movq	0x110(%rsp), %rcx
   251ef: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   251f2: 48 8b 0d 7f 37 05 00         	movq	0x5377f(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   251f9: 48 8b bc 24 50 01 00 00      	movq	0x150(%rsp), %rdi
   25201: ff d1                        	callq	*%rcx
   25203: 03 c3                        	addl	%ebx, %eax
   25205: 48 8b 8c 24 58 01 00 00      	movq	0x158(%rsp), %rcx
   2520d: 03 c8                        	addl	%eax, %ecx
   2520f: 48 89 8c 24 d8 00 00 00      	movq	%rcx, 0xd8(%rsp)
   25217: 48 8b 05 62 37 05 00         	movq	0x53762(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   2521e: 48 8b bc 24 e0 00 00 00      	movq	0xe0(%rsp), %rdi
   25226: ff d0                        	callq	*%rax
   25228: 48 8b 8c 24 08 01 00 00      	movq	0x108(%rsp), %rcx
   25230: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25233: 48 8b 0d 3e 37 05 00         	movq	0x5373e(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   2523a: 48 8b bc 24 48 01 00 00      	movq	0x148(%rsp), %rdi
   25242: ff d1                        	callq	*%rcx
   25244: 03 c3                        	addl	%ebx, %eax
   25246: 48 8b 8c 24 50 01 00 00      	movq	0x150(%rsp), %rcx
   2524e: 03 c8                        	addl	%eax, %ecx
   25250: 48 89 8c 24 d0 00 00 00      	movq	%rcx, 0xd0(%rsp)
   25258: 48 8b 05 21 37 05 00         	movq	0x53721(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   2525f: 48 8b bc 24 d8 00 00 00      	movq	0xd8(%rsp), %rdi
   25267: ff d0                        	callq	*%rax
   25269: 48 8b 8c 24 00 01 00 00      	movq	0x100(%rsp), %rcx
   25271: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25274: 48 8b 0d fd 36 05 00         	movq	0x536fd(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   2527b: 48 8b bc 24 40 01 00 00      	movq	0x140(%rsp), %rdi
   25283: ff d1                        	callq	*%rcx
   25285: 03 c3                        	addl	%ebx, %eax
   25287: 48 8b 8c 24 48 01 00 00      	movq	0x148(%rsp), %rcx
   2528f: 03 c8                        	addl	%eax, %ecx
   25291: 48 89 8c 24 c8 00 00 00      	movq	%rcx, 0xc8(%rsp)
   25299: 48 8b 05 e0 36 05 00         	movq	0x536e0(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   252a0: 48 8b bc 24 d0 00 00 00      	movq	0xd0(%rsp), %rdi
   252a8: ff d0                        	callq	*%rax
   252aa: 48 8b 8c 24 f8 00 00 00      	movq	0xf8(%rsp), %rcx
   252b2: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   252b5: 48 8b 0d bc 36 05 00         	movq	0x536bc(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   252bc: 48 8b bc 24 38 01 00 00      	movq	0x138(%rsp), %rdi
   252c4: ff d1                        	callq	*%rcx
   252c6: 03 c3                        	addl	%ebx, %eax
   252c8: 48 8b 8c 24 40 01 00 00      	movq	0x140(%rsp), %rcx
   252d0: 03 c8                        	addl	%eax, %ecx
   252d2: 48 89 8c 24 c0 00 00 00      	movq	%rcx, 0xc0(%rsp)
   252da: 48 8b 05 9f 36 05 00         	movq	0x5369f(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   252e1: 48 8b bc 24 c8 00 00 00      	movq	0xc8(%rsp), %rdi
   252e9: ff d0                        	callq	*%rax
   252eb: 48 8b 8c 24 f0 00 00 00      	movq	0xf0(%rsp), %rcx
   252f3: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   252f6: 48 8b 0d 7b 36 05 00         	movq	0x5367b(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   252fd: 48 8b bc 24 30 01 00 00      	movq	0x130(%rsp), %rdi
   25305: ff d1                        	callq	*%rcx
   25307: 03 c3                        	addl	%ebx, %eax
   25309: 48 8b 8c 24 38 01 00 00      	movq	0x138(%rsp), %rcx
   25311: 03 c8                        	addl	%eax, %ecx
   25313: 48 89 8c 24 b8 00 00 00      	movq	%rcx, 0xb8(%rsp)
   2531b: 48 8b 05 5e 36 05 00         	movq	0x5365e(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25322: 48 8b bc 24 c0 00 00 00      	movq	0xc0(%rsp), %rdi
   2532a: ff d0                        	callq	*%rax
   2532c: 48 8b 8c 24 e8 00 00 00      	movq	0xe8(%rsp), %rcx
   25334: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25337: 48 8b 0d 3a 36 05 00         	movq	0x5363a(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   2533e: 48 8b bc 24 28 01 00 00      	movq	0x128(%rsp), %rdi
   25346: ff d1                        	callq	*%rcx
   25348: 03 c3                        	addl	%ebx, %eax
   2534a: 48 8b 8c 24 30 01 00 00      	movq	0x130(%rsp), %rcx
   25352: 03 c8                        	addl	%eax, %ecx
   25354: 48 89 8c 24 b0 00 00 00      	movq	%rcx, 0xb0(%rsp)
   2535c: 48 8b 05 1d 36 05 00         	movq	0x5361d(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25363: 48 8b bc 24 b8 00 00 00      	movq	0xb8(%rsp), %rdi
   2536b: ff d0                        	callq	*%rax
   2536d: 48 8b 8c 24 e0 00 00 00      	movq	0xe0(%rsp), %rcx
   25375: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25378: 48 8b 0d f9 35 05 00         	movq	0x535f9(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   2537f: 48 8b bc 24 20 01 00 00      	movq	0x120(%rsp), %rdi
   25387: ff d1                        	callq	*%rcx
   25389: 03 c3                        	addl	%ebx, %eax
   2538b: 48 8b 8c 24 28 01 00 00      	movq	0x128(%rsp), %rcx
   25393: 03 c8                        	addl	%eax, %ecx
   25395: 48 89 8c 24 a8 00 00 00      	movq	%rcx, 0xa8(%rsp)
   2539d: 48 8b 05 dc 35 05 00         	movq	0x535dc(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   253a4: 48 8b bc 24 b0 00 00 00      	movq	0xb0(%rsp), %rdi
   253ac: ff d0                        	callq	*%rax
   253ae: 48 8b 8c 24 d8 00 00 00      	movq	0xd8(%rsp), %rcx
   253b6: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   253b9: 48 8b 0d b8 35 05 00         	movq	0x535b8(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   253c0: 48 8b bc 24 18 01 00 00      	movq	0x118(%rsp), %rdi
   253c8: ff d1                        	callq	*%rcx
   253ca: 03 c3                        	addl	%ebx, %eax
   253cc: 48 8b 8c 24 20 01 00 00      	movq	0x120(%rsp), %rcx
   253d4: 03 c8                        	addl	%eax, %ecx
   253d6: 48 89 8c 24 a0 00 00 00      	movq	%rcx, 0xa0(%rsp)
   253de: 48 8b 05 9b 35 05 00         	movq	0x5359b(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   253e5: 48 8b bc 24 a8 00 00 00      	movq	0xa8(%rsp), %rdi
   253ed: ff d0                        	callq	*%rax
   253ef: 48 8b 8c 24 d0 00 00 00      	movq	0xd0(%rsp), %rcx
   253f7: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   253fa: 48 8b 0d 77 35 05 00         	movq	0x53577(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25401: 48 8b bc 24 10 01 00 00      	movq	0x110(%rsp), %rdi
   25409: ff d1                        	callq	*%rcx
   2540b: 03 c3                        	addl	%ebx, %eax
   2540d: 48 8b 8c 24 18 01 00 00      	movq	0x118(%rsp), %rcx
   25415: 03 c8                        	addl	%eax, %ecx
   25417: 48 89 8c 24 98 00 00 00      	movq	%rcx, 0x98(%rsp)
   2541f: 48 8b 05 5a 35 05 00         	movq	0x5355a(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25426: 48 8b bc 24 a0 00 00 00      	movq	0xa0(%rsp), %rdi
   2542e: ff d0                        	callq	*%rax
   25430: 48 8b 8c 24 c8 00 00 00      	movq	0xc8(%rsp), %rcx
   25438: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   2543b: 48 8b 0d 36 35 05 00         	movq	0x53536(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25442: 48 8b bc 24 08 01 00 00      	movq	0x108(%rsp), %rdi
   2544a: ff d1                        	callq	*%rcx
   2544c: 03 c3                        	addl	%ebx, %eax
   2544e: 48 8b 8c 24 10 01 00 00      	movq	0x110(%rsp), %rcx
   25456: 03 c8                        	addl	%eax, %ecx
   25458: 48 89 8c 24 90 00 00 00      	movq	%rcx, 0x90(%rsp)
   25460: 48 8b 05 19 35 05 00         	movq	0x53519(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25467: 48 8b bc 24 98 00 00 00      	movq	0x98(%rsp), %rdi
   2546f: ff d0                        	callq	*%rax
   25471: 48 8b 8c 24 c0 00 00 00      	movq	0xc0(%rsp), %rcx
   25479: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   2547c: 48 8b 0d f5 34 05 00         	movq	0x534f5(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25483: 48 8b bc 24 00 01 00 00      	movq	0x100(%rsp), %rdi
   2548b: ff d1                        	callq	*%rcx
   2548d: 03 c3                        	addl	%ebx, %eax
   2548f: 48 8b 8c 24 08 01 00 00      	movq	0x108(%rsp), %rcx
   25497: 03 c8                        	addl	%eax, %ecx
   25499: 48 89 8c 24 88 00 00 00      	movq	%rcx, 0x88(%rsp)
   254a1: 48 8b 05 d8 34 05 00         	movq	0x534d8(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   254a8: 48 8b bc 24 90 00 00 00      	movq	0x90(%rsp), %rdi
   254b0: ff d0                        	callq	*%rax
   254b2: 48 8b 8c 24 b8 00 00 00      	movq	0xb8(%rsp), %rcx
   254ba: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   254bd: 48 8b 0d b4 34 05 00         	movq	0x534b4(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   254c4: 48 8b bc 24 f8 00 00 00      	movq	0xf8(%rsp), %rdi
   254cc: ff d1                        	callq	*%rcx
   254ce: 03 c3                        	addl	%ebx, %eax
   254d0: 48 8b 8c 24 00 01 00 00      	movq	0x100(%rsp), %rcx
   254d8: 03 c8                        	addl	%eax, %ecx
   254da: 48 89 8c 24 80 00 00 00      	movq	%rcx, 0x80(%rsp)
   254e2: 48 8b 05 97 34 05 00         	movq	0x53497(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   254e9: 48 8b bc 24 88 00 00 00      	movq	0x88(%rsp), %rdi
   254f1: ff d0                        	callq	*%rax
   254f3: 48 8b 8c 24 b0 00 00 00      	movq	0xb0(%rsp), %rcx
   254fb: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   254fe: 48 8b 0d 73 34 05 00         	movq	0x53473(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25505: 48 8b bc 24 f0 00 00 00      	movq	0xf0(%rsp), %rdi
   2550d: ff d1                        	callq	*%rcx
   2550f: 03 c3                        	addl	%ebx, %eax
   25511: 48 8b 8c 24 f8 00 00 00      	movq	0xf8(%rsp), %rcx
   25519: 03 c8                        	addl	%eax, %ecx
   2551b: 48 89 4c 24 78               	movq	%rcx, 0x78(%rsp)
   25520: 48 8b 05 59 34 05 00         	movq	0x53459(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25527: 48 8b bc 24 80 00 00 00      	movq	0x80(%rsp), %rdi
   2552f: ff d0                        	callq	*%rax
   25531: 48 8b 8c 24 a8 00 00 00      	movq	0xa8(%rsp), %rcx
   25539: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   2553c: 48 8b 0d 35 34 05 00         	movq	0x53435(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   25543: 48 8b bc 24 e8 00 00 00      	movq	0xe8(%rsp), %rdi
   2554b: ff d1                        	callq	*%rcx
   2554d: 03 c3                        	addl	%ebx, %eax
   2554f: 48 8b 8c 24 f0 00 00 00      	movq	0xf0(%rsp), %rcx
   25557: 03 c8                        	addl	%eax, %ecx
   25559: 48 89 4c 24 70               	movq	%rcx, 0x70(%rsp)
   2555e: 48 8b 05 1b 34 05 00         	movq	0x5341b(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25565: 48 8b 7c 24 78               	movq	0x78(%rsp), %rdi
   2556a: ff d0                        	callq	*%rax
   2556c: 48 8b 8c 24 a0 00 00 00      	movq	0xa0(%rsp), %rcx
   25574: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25577: 48 8b 0d fa 33 05 00         	movq	0x533fa(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   2557e: 48 8b bc 24 e0 00 00 00      	movq	0xe0(%rsp), %rdi
   25586: ff d1                        	callq	*%rcx
   25588: 03 c3                        	addl	%ebx, %eax
   2558a: 48 8b 8c 24 e8 00 00 00      	movq	0xe8(%rsp), %rcx
   25592: 03 c8                        	addl	%eax, %ecx
   25594: 48 89 4c 24 68               	movq	%rcx, 0x68(%rsp)
   25599: 48 8b 05 e0 33 05 00         	movq	0x533e0(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   255a0: 48 8b 7c 24 70               	movq	0x70(%rsp), %rdi
   255a5: ff d0                        	callq	*%rax
   255a7: 48 8b 8c 24 98 00 00 00      	movq	0x98(%rsp), %rcx
   255af: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   255b2: 48 8b 0d bf 33 05 00         	movq	0x533bf(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   255b9: 48 8b bc 24 d8 00 00 00      	movq	0xd8(%rsp), %rdi
   255c1: ff d1                        	callq	*%rcx
   255c3: 03 c3                        	addl	%ebx, %eax
   255c5: 48 8b 8c 24 e0 00 00 00      	movq	0xe0(%rsp), %rcx
   255cd: 03 c8                        	addl	%eax, %ecx
   255cf: 48 89 4c 24 60               	movq	%rcx, 0x60(%rsp)
   255d4: 48 8b 05 a5 33 05 00         	movq	0x533a5(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   255db: 48 8b 7c 24 68               	movq	0x68(%rsp), %rdi
   255e0: ff d0                        	callq	*%rax
   255e2: 48 8b 8c 24 90 00 00 00      	movq	0x90(%rsp), %rcx
   255ea: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   255ed: 48 8b 0d 84 33 05 00         	movq	0x53384(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   255f4: 48 8b bc 24 d0 00 00 00      	movq	0xd0(%rsp), %rdi
   255fc: ff d1                        	callq	*%rcx
   255fe: 03 c3                        	addl	%ebx, %eax
   25600: 48 8b 8c 24 d8 00 00 00      	movq	0xd8(%rsp), %rcx
   25608: 03 c8                        	addl	%eax, %ecx
   2560a: 48 89 4c 24 58               	movq	%rcx, 0x58(%rsp)
   2560f: 48 8b 05 6a 33 05 00         	movq	0x5336a(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25616: 48 8b 7c 24 60               	movq	0x60(%rsp), %rdi
   2561b: ff d0                        	callq	*%rax
   2561d: 48 8b 8c 24 88 00 00 00      	movq	0x88(%rsp), %rcx
   25625: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25628: 48 8b 0d 49 33 05 00         	movq	0x53349(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   2562f: 48 8b bc 24 c8 00 00 00      	movq	0xc8(%rsp), %rdi
   25637: ff d1                        	callq	*%rcx
   25639: 03 c3                        	addl	%ebx, %eax
   2563b: 48 8b 8c 24 d0 00 00 00      	movq	0xd0(%rsp), %rcx
   25643: 03 c8                        	addl	%eax, %ecx
   25645: 48 89 4c 24 50               	movq	%rcx, 0x50(%rsp)
   2564a: 48 8b 05 2f 33 05 00         	movq	0x5332f(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   25651: 48 8b 7c 24 58               	movq	0x58(%rsp), %rdi
   25656: ff d0                        	callq	*%rax
   25658: 48 8b 8c 24 80 00 00 00      	movq	0x80(%rsp), %rcx
   25660: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   25663: 48 8b 0d 0e 33 05 00         	movq	0x5330e(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   2566a: 48 8b bc 24 c0 00 00 00      	movq	0xc0(%rsp), %rdi
   25672: ff d1                        	callq	*%rcx
   25674: 03 c3                        	addl	%ebx, %eax
   25676: 48 8b 8c 24 c8 00 00 00      	movq	0xc8(%rsp), %rcx
   2567e: 03 c8                        	addl	%eax, %ecx
   25680: 48 89 4c 24 48               	movq	%rcx, 0x48(%rsp)
   25685: 48 8b 05 f4 32 05 00         	movq	0x532f4(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   2568c: 48 8b 7c 24 50               	movq	0x50(%rsp), %rdi
   25691: ff d0                        	callq	*%rax
   25693: 48 8b 4c 24 78               	movq	0x78(%rsp), %rcx
   25698: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   2569b: 48 8b 0d d6 32 05 00         	movq	0x532d6(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   256a2: 48 8b bc 24 b8 00 00 00      	movq	0xb8(%rsp), %rdi
   256aa: ff d1                        	callq	*%rcx
   256ac: 03 c3                        	addl	%ebx, %eax
   256ae: 48 8b 8c 24 c0 00 00 00      	movq	0xc0(%rsp), %rcx
   256b6: 03 c8                        	addl	%eax, %ecx
   256b8: 48 89 4c 24 40               	movq	%rcx, 0x40(%rsp)
   256bd: 48 8b 05 bc 32 05 00         	movq	0x532bc(%rip), %rax     # 0x78980 <sha256.gamma1$got>
   256c4: 48 8b 7c 24 48               	movq	0x48(%rsp), %rdi
   256c9: ff d0                        	callq	*%rax
   256cb: 48 8b 4c 24 70               	movq	0x70(%rsp), %rcx
   256d0: 8d 1c 08                     	leal	(%rax,%rcx), %ebx
   256d3: 48 8b 0d 9e 32 05 00         	movq	0x5329e(%rip), %rcx     # 0x78978 <sha256.gamma0$got>
   256da: 48 8b bc 24 b0 00 00 00      	movq	0xb0(%rsp), %rdi
   256e2: ff d1                        	callq	*%rcx
   256e4: 03 c3                        	addl	%ebx, %eax
   256e6: 48 8b 8c 24 b8 00 00 00      	movq	0xb8(%rsp), %rcx
   256ee: 03 c8                        	addl	%eax, %ecx
   256f0: 48 89 4c 24 38               	movq	%rcx, 0x38(%rsp)
   256f5: 48 8b 0c 24                  	movq	(%rsp), %rcx
   256f9: 8b 49 10                     	movl	0x10(%rcx), %ecx
   256fc: 48 89 4c 24 30               	movq	%rcx, 0x30(%rsp)
   25701: 48 8b 0c 24                  	movq	(%rsp), %rcx
   25705: 8b 49 14                     	movl	0x14(%rcx), %ecx
   25708: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   2570d: 48 8b 0c 24                  	movq	(%rsp), %rcx
   25711: 8b 49 18                     	movl	0x18(%rcx), %ecx
   25714: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   25719: 48 8b 0c 24                  	movq	(%rsp), %rcx
   2571d: 8b 59 1c                     	movl	0x1c(%rcx), %ebx
   25720: 44 8b 79 20                  	movl	0x20(%rcx), %r15d
   25724: 44 8b 61 24                  	movl	0x24(%rcx), %r12d
   25728: 44 8b 69 28                  	movl	0x28(%rcx), %r13d
   2572c: 44 8b 71 2c                  	movl	0x2c(%rcx), %r14d
   25730: 48 8b 05 39 32 05 00         	movq	0x53239(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25737: 4c 89 ff                     	movq	%r15, %rdi
   2573a: ff d0                        	callq	*%rax
   2573c: 44 03 f0                     	addl	%eax, %r14d
   2573f: 48 8b 05 12 32 05 00         	movq	0x53212(%rip), %rax     # 0x78958 <sha256.ch$got>
   25746: 4c 89 ea                     	movq	%r13, %rdx
   25749: 4c 89 e6                     	movq	%r12, %rsi
   2574c: 4c 89 ff                     	movq	%r15, %rdi
   2574f: ff d0                        	callq	*%rax
   25751: 44 03 f0                     	addl	%eax, %r14d
   25754: 48 8b 05 f5 2f 05 00         	movq	0x52ff5(%rip), %rax     # 0x78750 <sha256.k0$got>
   2575b: ff d0                        	callq	*%rax
   2575d: 41 03 c6                     	addl	%r14d, %eax
   25760: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   25765: 44 8d 34 10                  	leal	(%rax,%rdx), %r14d
   25769: 48 8b 05 f8 31 05 00         	movq	0x531f8(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25770: 48 8b 7c 24 30               	movq	0x30(%rsp), %rdi
   25775: ff d0                        	callq	*%rax
   25777: 48 89 44 24 20               	movq	%rax, 0x20(%rsp)
   2577c: 48 8b 0d dd 31 05 00         	movq	0x531dd(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25783: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   25788: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   2578d: 48 8b 7c 24 30               	movq	0x30(%rsp), %rdi
   25792: ff d1                        	callq	*%rcx
   25794: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   25799: 03 c8                        	addl	%eax, %ecx
   2579b: 41 03 de                     	addl	%r14d, %ebx
   2579e: 41 03 ce                     	addl	%r14d, %ecx
   257a1: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   257a6: 48 8b 0d c3 31 05 00         	movq	0x531c3(%rip), %rcx     # 0x78970 <sha256.sigma1$got>
   257ad: 48 89 df                     	movq	%rbx, %rdi
   257b0: ff d1                        	callq	*%rcx
   257b2: 48 89 5c 24 20               	movq	%rbx, 0x20(%rsp)
   257b7: 41 8d 5c 05 00               	leal	(%r13,%rax), %ebx
   257bc: 48 8b 05 95 31 05 00         	movq	0x53195(%rip), %rax     # 0x78958 <sha256.ch$got>
   257c3: 4c 89 e2                     	movq	%r12, %rdx
   257c6: 4c 89 fe                     	movq	%r15, %rsi
   257c9: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   257ce: ff d0                        	callq	*%rax
   257d0: 03 d8                        	addl	%eax, %ebx
   257d2: 48 8b 05 7f 2f 05 00         	movq	0x52f7f(%rip), %rax     # 0x78758 <sha256.k1$got>
   257d9: ff d0                        	callq	*%rax
   257db: 03 c3                        	addl	%ebx, %eax
   257dd: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   257e2: 44 8d 34 08                  	leal	(%rax,%rcx), %r14d
   257e6: 48 8b 05 7b 31 05 00         	movq	0x5317b(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   257ed: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   257f2: ff d0                        	callq	*%rax
   257f4: 48 89 c3                     	movq	%rax, %rbx
   257f7: 48 8b 0d 62 31 05 00         	movq	0x53162(%rip), %rcx     # 0x78960 <sha256.maj$got>
   257fe: 4c 8b 6c 24 08               	movq	0x8(%rsp), %r13
   25803: 4c 89 ea                     	movq	%r13, %rdx
   25806: 48 8b 74 24 30               	movq	0x30(%rsp), %rsi
   2580b: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   25810: ff d1                        	callq	*%rcx
   25812: 48 89 de                     	movq	%rbx, %rsi
   25815: 03 c6                        	addl	%esi, %eax
   25817: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   2581c: 42 8d 1c 31                  	leal	(%rcx,%r14), %ebx
   25820: 41 8d 0c 06                  	leal	(%r14,%rax), %ecx
   25824: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   25829: 48 8b 05 40 31 05 00         	movq	0x53140(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25830: 48 89 df                     	movq	%rbx, %rdi
   25833: ff d0                        	callq	*%rax
   25835: 44 03 e0                     	addl	%eax, %r12d
   25838: 48 8b 05 19 31 05 00         	movq	0x53119(%rip), %rax     # 0x78958 <sha256.ch$got>
   2583f: 4c 89 fa                     	movq	%r15, %rdx
   25842: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   25847: 48 89 df                     	movq	%rbx, %rdi
   2584a: ff d0                        	callq	*%rax
   2584c: 44 03 e0                     	addl	%eax, %r12d
   2584f: 48 8b 05 0a 2f 05 00         	movq	0x52f0a(%rip), %rax     # 0x78760 <sha256.k2$got>
   25856: ff d0                        	callq	*%rax
   25858: 41 03 c4                     	addl	%r12d, %eax
   2585b: 48 8b 8c 24 20 02 00 00      	movq	0x220(%rsp), %rcx
   25863: 44 8d 34 08                  	leal	(%rax,%rcx), %r14d
   25867: 48 8b 05 fa 30 05 00         	movq	0x530fa(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2586e: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   25873: ff d0                        	callq	*%rax
   25875: 49 89 c4                     	movq	%rax, %r12
   25878: 48 8b 0d e1 30 05 00         	movq	0x530e1(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2587f: 48 8b 54 24 30               	movq	0x30(%rsp), %rdx
   25884: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   25889: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   2588e: ff d1                        	callq	*%rcx
   25890: 4c 89 e1                     	movq	%r12, %rcx
   25893: 03 c1                        	addl	%ecx, %eax
   25895: 47 8d 64 35 00               	leal	(%r13,%r14), %r12d
   2589a: 41 8d 0c 06                  	leal	(%r14,%rax), %ecx
   2589e: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   258a3: 48 8b 05 c6 30 05 00         	movq	0x530c6(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   258aa: 4c 89 e7                     	movq	%r12, %rdi
   258ad: ff d0                        	callq	*%rax
   258af: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   258b3: 48 8b 0d 9e 30 05 00         	movq	0x5309e(%rip), %rcx     # 0x78958 <sha256.ch$got>
   258ba: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   258bf: 48 89 de                     	movq	%rbx, %rsi
   258c2: 4c 89 e7                     	movq	%r12, %rdi
   258c5: ff d1                        	callq	*%rcx
   258c7: 44 03 e8                     	addl	%eax, %r13d
   258ca: 48 8b 0d 97 2e 05 00         	movq	0x52e97(%rip), %rcx     # 0x78768 <sha256.k3$got>
   258d1: ff d1                        	callq	*%rcx
   258d3: 41 03 c5                     	addl	%r13d, %eax
   258d6: 48 8b 8c 24 18 02 00 00      	movq	0x218(%rsp), %rcx
   258de: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   258e2: 48 8b 05 7f 30 05 00         	movq	0x5307f(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   258e9: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   258ee: ff d0                        	callq	*%rax
   258f0: 49 89 c6                     	movq	%rax, %r14
   258f3: 48 8b 0d 66 30 05 00         	movq	0x53066(%rip), %rcx     # 0x78960 <sha256.maj$got>
   258fa: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   258ff: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   25904: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   25909: ff d1                        	callq	*%rcx
   2590b: 4c 89 f1                     	movq	%r14, %rcx
   2590e: 03 c1                        	addl	%ecx, %eax
   25910: 48 8b 4c 24 30               	movq	0x30(%rsp), %rcx
   25915: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   25919: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2591e: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   25923: 48 8b 05 46 30 05 00         	movq	0x53046(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2592a: 4c 89 ff                     	movq	%r15, %rdi
   2592d: ff d0                        	callq	*%rax
   2592f: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   25934: 44 8d 2c 01                  	leal	(%rcx,%rax), %r13d
   25938: 48 8b 0d 19 30 05 00         	movq	0x53019(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2593f: 48 89 da                     	movq	%rbx, %rdx
   25942: 4c 89 e6                     	movq	%r12, %rsi
   25945: 4c 89 ff                     	movq	%r15, %rdi
   25948: ff d1                        	callq	*%rcx
   2594a: 44 03 e8                     	addl	%eax, %r13d
   2594d: 48 8b 0d 1c 2e 05 00         	movq	0x52e1c(%rip), %rcx     # 0x78770 <sha256.k4$got>
   25954: ff d1                        	callq	*%rcx
   25956: 41 03 c5                     	addl	%r13d, %eax
   25959: 48 8b 8c 24 10 02 00 00      	movq	0x210(%rsp), %rcx
   25961: 44 8d 34 08                  	leal	(%rax,%rcx), %r14d
   25965: 48 8b 05 fc 2f 05 00         	movq	0x52ffc(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2596c: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   25971: ff d0                        	callq	*%rax
   25973: 49 89 c5                     	movq	%rax, %r13
   25976: 48 8b 0d e3 2f 05 00         	movq	0x52fe3(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2597d: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   25982: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   25987: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   2598c: ff d1                        	callq	*%rcx
   2598e: 4c 89 e9                     	movq	%r13, %rcx
   25991: 03 c8                        	addl	%eax, %ecx
   25993: 48 8b 44 24 18               	movq	0x18(%rsp), %rax
   25998: 46 8d 2c 30                  	leal	(%rax,%r14), %r13d
   2599c: 44 03 f1                     	addl	%ecx, %r14d
   2599f: 48 8b 0d ca 2f 05 00         	movq	0x52fca(%rip), %rcx     # 0x78970 <sha256.sigma1$got>
   259a6: 4c 89 ef                     	movq	%r13, %rdi
   259a9: ff d1                        	callq	*%rcx
   259ab: 4c 89 6c 24 20               	movq	%r13, 0x20(%rsp)
   259b0: 03 d8                        	addl	%eax, %ebx
   259b2: 48 8b 05 9f 2f 05 00         	movq	0x52f9f(%rip), %rax     # 0x78958 <sha256.ch$got>
   259b9: 4c 89 e2                     	movq	%r12, %rdx
   259bc: 4c 89 fe                     	movq	%r15, %rsi
   259bf: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   259c4: ff d0                        	callq	*%rax
   259c6: 03 d8                        	addl	%eax, %ebx
   259c8: 48 8b 05 a9 2d 05 00         	movq	0x52da9(%rip), %rax     # 0x78778 <sha256.k5$got>
   259cf: ff d0                        	callq	*%rax
   259d1: 03 c3                        	addl	%ebx, %eax
   259d3: 48 8b 94 24 08 02 00 00      	movq	0x208(%rsp), %rdx
   259db: 44 8d 2c 10                  	leal	(%rax,%rdx), %r13d
   259df: 48 8b 05 82 2f 05 00         	movq	0x52f82(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   259e6: 4c 89 f7                     	movq	%r14, %rdi
   259e9: ff d0                        	callq	*%rax
   259eb: 48 89 c3                     	movq	%rax, %rbx
   259ee: 48 8b 0d 6b 2f 05 00         	movq	0x52f6b(%rip), %rcx     # 0x78960 <sha256.maj$got>
   259f5: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   259fa: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   259ff: 4c 89 f7                     	movq	%r14, %rdi
   25a02: ff d1                        	callq	*%rcx
   25a04: 48 89 d9                     	movq	%rbx, %rcx
   25a07: 03 c1                        	addl	%ecx, %eax
   25a09: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   25a0e: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   25a12: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25a17: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   25a1c: 48 8b 05 4d 2f 05 00         	movq	0x52f4d(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25a23: 48 89 df                     	movq	%rbx, %rdi
   25a26: ff d0                        	callq	*%rax
   25a28: 44 03 e0                     	addl	%eax, %r12d
   25a2b: 48 8b 05 26 2f 05 00         	movq	0x52f26(%rip), %rax     # 0x78958 <sha256.ch$got>
   25a32: 4c 89 fa                     	movq	%r15, %rdx
   25a35: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   25a3a: 48 89 df                     	movq	%rbx, %rdi
   25a3d: ff d0                        	callq	*%rax
   25a3f: 44 03 e0                     	addl	%eax, %r12d
   25a42: 48 8b 05 37 2d 05 00         	movq	0x52d37(%rip), %rax     # 0x78780 <sha256.k6$got>
   25a49: ff d0                        	callq	*%rax
   25a4b: 41 03 c4                     	addl	%r12d, %eax
   25a4e: 48 8b b4 24 00 02 00 00      	movq	0x200(%rsp), %rsi
   25a56: 44 8d 2c 30                  	leal	(%rax,%rsi), %r13d
   25a5a: 48 8b 05 07 2f 05 00         	movq	0x52f07(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25a61: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   25a66: ff d0                        	callq	*%rax
   25a68: 49 89 c4                     	movq	%rax, %r12
   25a6b: 48 8b 0d ee 2e 05 00         	movq	0x52eee(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25a72: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   25a77: 4c 89 f6                     	movq	%r14, %rsi
   25a7a: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   25a7f: ff d1                        	callq	*%rcx
   25a81: 4c 89 e1                     	movq	%r12, %rcx
   25a84: 03 c1                        	addl	%ecx, %eax
   25a86: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   25a8b: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   25a8f: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25a94: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   25a99: 48 8b 05 d0 2e 05 00         	movq	0x52ed0(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25aa0: 4c 89 e7                     	movq	%r12, %rdi
   25aa3: ff d0                        	callq	*%rax
   25aa5: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   25aa9: 48 8b 0d a8 2e 05 00         	movq	0x52ea8(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25ab0: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   25ab5: 48 89 de                     	movq	%rbx, %rsi
   25ab8: 4c 89 e7                     	movq	%r12, %rdi
   25abb: ff d1                        	callq	*%rcx
   25abd: 44 03 e8                     	addl	%eax, %r13d
   25ac0: 48 8b 0d c1 2c 05 00         	movq	0x52cc1(%rip), %rcx     # 0x78788 <sha256.k7$got>
   25ac7: ff d1                        	callq	*%rcx
   25ac9: 41 03 c5                     	addl	%r13d, %eax
   25acc: 4c 8b 84 24 f8 01 00 00      	movq	0x1f8(%rsp), %r8
   25ad4: 46 8d 2c 00                  	leal	(%rax,%r8), %r13d
   25ad8: 48 8b 05 89 2e 05 00         	movq	0x52e89(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25adf: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   25ae4: ff d0                        	callq	*%rax
   25ae6: 49 89 c7                     	movq	%rax, %r15
   25ae9: 48 8b 0d 70 2e 05 00         	movq	0x52e70(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25af0: 4c 89 f2                     	movq	%r14, %rdx
   25af3: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   25af8: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   25afd: ff d1                        	callq	*%rcx
   25aff: 4c 89 f9                     	movq	%r15, %rcx
   25b02: 03 c1                        	addl	%ecx, %eax
   25b04: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   25b09: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   25b0d: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25b12: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   25b17: 48 8b 05 52 2e 05 00         	movq	0x52e52(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25b1e: 4c 89 ff                     	movq	%r15, %rdi
   25b21: ff d0                        	callq	*%rax
   25b23: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   25b28: 44 8d 2c 01                  	leal	(%rcx,%rax), %r13d
   25b2c: 48 8b 0d 25 2e 05 00         	movq	0x52e25(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25b33: 48 89 da                     	movq	%rbx, %rdx
   25b36: 4c 89 e6                     	movq	%r12, %rsi
   25b39: 4c 89 ff                     	movq	%r15, %rdi
   25b3c: ff d1                        	callq	*%rcx
   25b3e: 44 03 e8                     	addl	%eax, %r13d
   25b41: 48 8b 0d 48 2c 05 00         	movq	0x52c48(%rip), %rcx     # 0x78790 <sha256.k8$got>
   25b48: ff d1                        	callq	*%rcx
   25b4a: 41 03 c5                     	addl	%r13d, %eax
   25b4d: 48 8b 8c 24 f0 01 00 00      	movq	0x1f0(%rsp), %rcx
   25b55: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25b59: 48 8b 05 08 2e 05 00         	movq	0x52e08(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25b60: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   25b65: ff d0                        	callq	*%rax
   25b67: 48 89 44 24 28               	movq	%rax, 0x28(%rsp)
   25b6c: 48 8b 0d ed 2d 05 00         	movq	0x52ded(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25b73: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   25b78: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   25b7d: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   25b82: ff d1                        	callq	*%rcx
   25b84: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   25b89: 03 c1                        	addl	%ecx, %eax
   25b8b: 45 03 f5                     	addl	%r13d, %r14d
   25b8e: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25b93: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   25b98: 48 8b 05 d1 2d 05 00         	movq	0x52dd1(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25b9f: 4c 89 f7                     	movq	%r14, %rdi
   25ba2: ff d0                        	callq	*%rax
   25ba4: 03 d8                        	addl	%eax, %ebx
   25ba6: 48 8b 05 ab 2d 05 00         	movq	0x52dab(%rip), %rax     # 0x78958 <sha256.ch$got>
   25bad: 4c 89 e2                     	movq	%r12, %rdx
   25bb0: 4c 89 fe                     	movq	%r15, %rsi
   25bb3: 4c 89 f7                     	movq	%r14, %rdi
   25bb6: ff d0                        	callq	*%rax
   25bb8: 03 d8                        	addl	%eax, %ebx
   25bba: 48 8b 05 d7 2b 05 00         	movq	0x52bd7(%rip), %rax     # 0x78798 <sha256.k9$got>
   25bc1: ff d0                        	callq	*%rax
   25bc3: 03 c3                        	addl	%ebx, %eax
   25bc5: 48 8b 8c 24 e8 01 00 00      	movq	0x1e8(%rsp), %rcx
   25bcd: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25bd1: 48 8b 05 90 2d 05 00         	movq	0x52d90(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25bd8: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   25bdd: ff d0                        	callq	*%rax
   25bdf: 48 89 c3                     	movq	%rax, %rbx
   25be2: 48 8b 0d 77 2d 05 00         	movq	0x52d77(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25be9: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   25bee: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   25bf3: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   25bf8: ff d1                        	callq	*%rcx
   25bfa: 48 89 d9                     	movq	%rbx, %rcx
   25bfd: 03 c1                        	addl	%ecx, %eax
   25bff: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   25c04: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   25c08: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25c0d: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   25c12: 48 8b 05 57 2d 05 00         	movq	0x52d57(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25c19: 48 89 df                     	movq	%rbx, %rdi
   25c1c: ff d0                        	callq	*%rax
   25c1e: 44 03 e0                     	addl	%eax, %r12d
   25c21: 48 8b 0d 30 2d 05 00         	movq	0x52d30(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25c28: 4c 89 fa                     	movq	%r15, %rdx
   25c2b: 4c 89 f6                     	movq	%r14, %rsi
   25c2e: 48 89 df                     	movq	%rbx, %rdi
   25c31: ff d1                        	callq	*%rcx
   25c33: 44 03 e0                     	addl	%eax, %r12d
   25c36: 48 8b 0d 63 2b 05 00         	movq	0x52b63(%rip), %rcx     # 0x787a0 <sha256.k10$got>
   25c3d: ff d1                        	callq	*%rcx
   25c3f: 41 03 c4                     	addl	%r12d, %eax
   25c42: 48 8b 8c 24 e0 01 00 00      	movq	0x1e0(%rsp), %rcx
   25c4a: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25c4e: 48 8b 05 13 2d 05 00         	movq	0x52d13(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25c55: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   25c5a: ff d0                        	callq	*%rax
   25c5c: 49 89 c4                     	movq	%rax, %r12
   25c5f: 48 8b 0d fa 2c 05 00         	movq	0x52cfa(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25c66: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   25c6b: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   25c70: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   25c75: ff d1                        	callq	*%rcx
   25c77: 4c 89 e1                     	movq	%r12, %rcx
   25c7a: 03 c1                        	addl	%ecx, %eax
   25c7c: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   25c81: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   25c85: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25c8a: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   25c8f: 48 8b 05 da 2c 05 00         	movq	0x52cda(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25c96: 4c 89 e7                     	movq	%r12, %rdi
   25c99: ff d0                        	callq	*%rax
   25c9b: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   25c9f: 48 8b 0d b2 2c 05 00         	movq	0x52cb2(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25ca6: 4c 89 f2                     	movq	%r14, %rdx
   25ca9: 48 89 de                     	movq	%rbx, %rsi
   25cac: 4c 89 e7                     	movq	%r12, %rdi
   25caf: ff d1                        	callq	*%rcx
   25cb1: 44 03 e8                     	addl	%eax, %r13d
   25cb4: 48 8b 0d ed 2a 05 00         	movq	0x52aed(%rip), %rcx     # 0x787a8 <sha256.k11$got>
   25cbb: ff d1                        	callq	*%rcx
   25cbd: 41 03 c5                     	addl	%r13d, %eax
   25cc0: 48 8b 8c 24 d8 01 00 00      	movq	0x1d8(%rsp), %rcx
   25cc8: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25ccc: 48 8b 05 95 2c 05 00         	movq	0x52c95(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25cd3: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   25cd8: ff d0                        	callq	*%rax
   25cda: 49 89 c7                     	movq	%rax, %r15
   25cdd: 48 8b 0d 7c 2c 05 00         	movq	0x52c7c(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25ce4: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   25ce9: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   25cee: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   25cf3: ff d1                        	callq	*%rcx
   25cf5: 4c 89 ff                     	movq	%r15, %rdi
   25cf8: 03 c7                        	addl	%edi, %eax
   25cfa: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   25cff: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   25d03: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25d08: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   25d0d: 48 8b 05 5c 2c 05 00         	movq	0x52c5c(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25d14: 4c 89 ff                     	movq	%r15, %rdi
   25d17: ff d0                        	callq	*%rax
   25d19: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   25d1d: 48 8b 0d 34 2c 05 00         	movq	0x52c34(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25d24: 48 89 da                     	movq	%rbx, %rdx
   25d27: 4c 89 e6                     	movq	%r12, %rsi
   25d2a: 4c 89 ff                     	movq	%r15, %rdi
   25d2d: ff d1                        	callq	*%rcx
   25d2f: 44 03 e8                     	addl	%eax, %r13d
   25d32: 48 8b 0d 77 2a 05 00         	movq	0x52a77(%rip), %rcx     # 0x787b0 <sha256.k12$got>
   25d39: ff d1                        	callq	*%rcx
   25d3b: 41 03 c5                     	addl	%r13d, %eax
   25d3e: 48 8b 8c 24 d0 01 00 00      	movq	0x1d0(%rsp), %rcx
   25d46: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25d4a: 48 8b 05 17 2c 05 00         	movq	0x52c17(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25d51: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   25d56: ff d0                        	callq	*%rax
   25d58: 49 89 c6                     	movq	%rax, %r14
   25d5b: 48 8b 0d fe 2b 05 00         	movq	0x52bfe(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25d62: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   25d67: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   25d6c: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   25d71: ff d1                        	callq	*%rcx
   25d73: 4c 89 f1                     	movq	%r14, %rcx
   25d76: 03 c1                        	addl	%ecx, %eax
   25d78: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   25d7d: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   25d81: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25d86: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   25d8b: 48 8b 05 de 2b 05 00         	movq	0x52bde(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25d92: 4c 89 f7                     	movq	%r14, %rdi
   25d95: ff d0                        	callq	*%rax
   25d97: 03 d8                        	addl	%eax, %ebx
   25d99: 48 8b 05 b8 2b 05 00         	movq	0x52bb8(%rip), %rax     # 0x78958 <sha256.ch$got>
   25da0: 4c 89 e2                     	movq	%r12, %rdx
   25da3: 4c 89 fe                     	movq	%r15, %rsi
   25da6: 4c 89 f7                     	movq	%r14, %rdi
   25da9: ff d0                        	callq	*%rax
   25dab: 03 d8                        	addl	%eax, %ebx
   25dad: 48 8b 05 04 2a 05 00         	movq	0x52a04(%rip), %rax     # 0x787b8 <sha256.k13$got>
   25db4: ff d0                        	callq	*%rax
   25db6: 03 c3                        	addl	%ebx, %eax
   25db8: 48 8b 8c 24 c8 01 00 00      	movq	0x1c8(%rsp), %rcx
   25dc0: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25dc4: 48 8b 05 9d 2b 05 00         	movq	0x52b9d(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25dcb: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   25dd0: ff d0                        	callq	*%rax
   25dd2: 48 89 c3                     	movq	%rax, %rbx
   25dd5: 48 8b 0d 84 2b 05 00         	movq	0x52b84(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25ddc: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   25de1: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   25de6: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   25deb: ff d1                        	callq	*%rcx
   25ded: 48 89 d9                     	movq	%rbx, %rcx
   25df0: 03 c1                        	addl	%ecx, %eax
   25df2: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   25df7: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   25dfb: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25e00: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   25e05: 48 8b 05 64 2b 05 00         	movq	0x52b64(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25e0c: 48 89 df                     	movq	%rbx, %rdi
   25e0f: ff d0                        	callq	*%rax
   25e11: 44 03 e0                     	addl	%eax, %r12d
   25e14: 48 8b 0d 3d 2b 05 00         	movq	0x52b3d(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25e1b: 4c 89 fa                     	movq	%r15, %rdx
   25e1e: 4c 89 f6                     	movq	%r14, %rsi
   25e21: 48 89 df                     	movq	%rbx, %rdi
   25e24: ff d1                        	callq	*%rcx
   25e26: 44 03 e0                     	addl	%eax, %r12d
   25e29: 48 8b 0d 90 29 05 00         	movq	0x52990(%rip), %rcx     # 0x787c0 <sha256.k14$got>
   25e30: ff d1                        	callq	*%rcx
   25e32: 41 03 c4                     	addl	%r12d, %eax
   25e35: 48 8b 8c 24 c0 01 00 00      	movq	0x1c0(%rsp), %rcx
   25e3d: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25e41: 48 8b 05 20 2b 05 00         	movq	0x52b20(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25e48: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   25e4d: ff d0                        	callq	*%rax
   25e4f: 49 89 c4                     	movq	%rax, %r12
   25e52: 48 8b 0d 07 2b 05 00         	movq	0x52b07(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25e59: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   25e5e: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   25e63: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   25e68: ff d1                        	callq	*%rcx
   25e6a: 4c 89 e1                     	movq	%r12, %rcx
   25e6d: 03 c1                        	addl	%ecx, %eax
   25e6f: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   25e74: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   25e78: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25e7d: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   25e82: 48 8b 05 e7 2a 05 00         	movq	0x52ae7(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25e89: 4c 89 e7                     	movq	%r12, %rdi
   25e8c: ff d0                        	callq	*%rax
   25e8e: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   25e92: 48 8b 0d bf 2a 05 00         	movq	0x52abf(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25e99: 4c 89 f2                     	movq	%r14, %rdx
   25e9c: 48 89 de                     	movq	%rbx, %rsi
   25e9f: 4c 89 e7                     	movq	%r12, %rdi
   25ea2: ff d1                        	callq	*%rcx
   25ea4: 44 03 e8                     	addl	%eax, %r13d
   25ea7: 48 8b 0d 1a 29 05 00         	movq	0x5291a(%rip), %rcx     # 0x787c8 <sha256.k15$got>
   25eae: ff d1                        	callq	*%rcx
   25eb0: 41 03 c5                     	addl	%r13d, %eax
   25eb3: 48 8b b4 24 b8 01 00 00      	movq	0x1b8(%rsp), %rsi
   25ebb: 44 8d 2c 30                  	leal	(%rax,%rsi), %r13d
   25ebf: 48 8b 05 a2 2a 05 00         	movq	0x52aa2(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25ec6: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   25ecb: ff d0                        	callq	*%rax
   25ecd: 49 89 c7                     	movq	%rax, %r15
   25ed0: 48 8b 0d 89 2a 05 00         	movq	0x52a89(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25ed7: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   25edc: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   25ee1: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   25ee6: ff d1                        	callq	*%rcx
   25ee8: 4c 89 f9                     	movq	%r15, %rcx
   25eeb: 03 c1                        	addl	%ecx, %eax
   25eed: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   25ef2: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   25ef6: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25efb: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   25f00: 48 8b 05 69 2a 05 00         	movq	0x52a69(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25f07: 4c 89 ff                     	movq	%r15, %rdi
   25f0a: ff d0                        	callq	*%rax
   25f0c: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   25f10: 48 8b 0d 41 2a 05 00         	movq	0x52a41(%rip), %rcx     # 0x78958 <sha256.ch$got>
   25f17: 48 89 da                     	movq	%rbx, %rdx
   25f1a: 4c 89 e6                     	movq	%r12, %rsi
   25f1d: 4c 89 ff                     	movq	%r15, %rdi
   25f20: ff d1                        	callq	*%rcx
   25f22: 44 03 e8                     	addl	%eax, %r13d
   25f25: 48 8b 0d a4 28 05 00         	movq	0x528a4(%rip), %rcx     # 0x787d0 <sha256.k16$got>
   25f2c: ff d1                        	callq	*%rcx
   25f2e: 41 03 c5                     	addl	%r13d, %eax
   25f31: 48 8b 8c 24 b0 01 00 00      	movq	0x1b0(%rsp), %rcx
   25f39: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25f3d: 48 8b 05 24 2a 05 00         	movq	0x52a24(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25f44: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   25f49: ff d0                        	callq	*%rax
   25f4b: 49 89 c6                     	movq	%rax, %r14
   25f4e: 48 8b 0d 0b 2a 05 00         	movq	0x52a0b(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25f55: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   25f5a: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   25f5f: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   25f64: ff d1                        	callq	*%rcx
   25f66: 4c 89 f1                     	movq	%r14, %rcx
   25f69: 03 c1                        	addl	%ecx, %eax
   25f6b: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   25f70: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   25f74: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25f79: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   25f7e: 48 8b 05 eb 29 05 00         	movq	0x529eb(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25f85: 4c 89 f7                     	movq	%r14, %rdi
   25f88: ff d0                        	callq	*%rax
   25f8a: 03 d8                        	addl	%eax, %ebx
   25f8c: 48 8b 05 c5 29 05 00         	movq	0x529c5(%rip), %rax     # 0x78958 <sha256.ch$got>
   25f93: 4c 89 e2                     	movq	%r12, %rdx
   25f96: 4c 89 fe                     	movq	%r15, %rsi
   25f99: 4c 89 f7                     	movq	%r14, %rdi
   25f9c: ff d0                        	callq	*%rax
   25f9e: 03 d8                        	addl	%eax, %ebx
   25fa0: 48 8b 05 31 28 05 00         	movq	0x52831(%rip), %rax     # 0x787d8 <sha256.k17$got>
   25fa7: ff d0                        	callq	*%rax
   25fa9: 03 c3                        	addl	%ebx, %eax
   25fab: 48 8b 8c 24 a8 01 00 00      	movq	0x1a8(%rsp), %rcx
   25fb3: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   25fb7: 48 8b 05 aa 29 05 00         	movq	0x529aa(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   25fbe: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   25fc3: ff d0                        	callq	*%rax
   25fc5: 48 89 c3                     	movq	%rax, %rbx
   25fc8: 48 8b 0d 91 29 05 00         	movq	0x52991(%rip), %rcx     # 0x78960 <sha256.maj$got>
   25fcf: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   25fd4: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   25fd9: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   25fde: ff d1                        	callq	*%rcx
   25fe0: 48 89 d9                     	movq	%rbx, %rcx
   25fe3: 03 c1                        	addl	%ecx, %eax
   25fe5: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   25fea: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   25fee: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   25ff3: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   25ff8: 48 8b 05 71 29 05 00         	movq	0x52971(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   25fff: 48 89 df                     	movq	%rbx, %rdi
   26002: ff d0                        	callq	*%rax
   26004: 44 03 e0                     	addl	%eax, %r12d
   26007: 48 8b 0d 4a 29 05 00         	movq	0x5294a(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2600e: 4c 89 fa                     	movq	%r15, %rdx
   26011: 4c 89 f6                     	movq	%r14, %rsi
   26014: 48 89 df                     	movq	%rbx, %rdi
   26017: ff d1                        	callq	*%rcx
   26019: 44 03 e0                     	addl	%eax, %r12d
   2601c: 48 8b 0d bd 27 05 00         	movq	0x527bd(%rip), %rcx     # 0x787e0 <sha256.k18$got>
   26023: ff d1                        	callq	*%rcx
   26025: 41 03 c4                     	addl	%r12d, %eax
   26028: 48 8b 8c 24 a0 01 00 00      	movq	0x1a0(%rsp), %rcx
   26030: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26034: 48 8b 05 2d 29 05 00         	movq	0x5292d(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2603b: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26040: ff d0                        	callq	*%rax
   26042: 49 89 c4                     	movq	%rax, %r12
   26045: 48 8b 0d 14 29 05 00         	movq	0x52914(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2604c: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   26051: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   26056: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   2605b: ff d1                        	callq	*%rcx
   2605d: 4c 89 e1                     	movq	%r12, %rcx
   26060: 03 c1                        	addl	%ecx, %eax
   26062: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   26067: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   2606b: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26070: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   26075: 48 8b 05 f4 28 05 00         	movq	0x528f4(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2607c: 4c 89 e7                     	movq	%r12, %rdi
   2607f: ff d0                        	callq	*%rax
   26081: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   26085: 48 8b 0d cc 28 05 00         	movq	0x528cc(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2608c: 4c 89 f2                     	movq	%r14, %rdx
   2608f: 48 89 de                     	movq	%rbx, %rsi
   26092: 4c 89 e7                     	movq	%r12, %rdi
   26095: ff d1                        	callq	*%rcx
   26097: 44 03 e8                     	addl	%eax, %r13d
   2609a: 48 8b 0d 47 27 05 00         	movq	0x52747(%rip), %rcx     # 0x787e8 <sha256.k19$got>
   260a1: ff d1                        	callq	*%rcx
   260a3: 41 03 c5                     	addl	%r13d, %eax
   260a6: 48 8b 8c 24 98 01 00 00      	movq	0x198(%rsp), %rcx
   260ae: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   260b2: 48 8b 05 af 28 05 00         	movq	0x528af(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   260b9: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   260be: ff d0                        	callq	*%rax
   260c0: 49 89 c7                     	movq	%rax, %r15
   260c3: 48 8b 0d 96 28 05 00         	movq	0x52896(%rip), %rcx     # 0x78960 <sha256.maj$got>
   260ca: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   260cf: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   260d4: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   260d9: ff d1                        	callq	*%rcx
   260db: 4c 89 f9                     	movq	%r15, %rcx
   260de: 03 c1                        	addl	%ecx, %eax
   260e0: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   260e5: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   260e9: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   260ee: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   260f3: 48 8b 05 76 28 05 00         	movq	0x52876(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   260fa: 4c 89 ff                     	movq	%r15, %rdi
   260fd: ff d0                        	callq	*%rax
   260ff: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   26103: 48 8b 0d 4e 28 05 00         	movq	0x5284e(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2610a: 48 89 da                     	movq	%rbx, %rdx
   2610d: 4c 89 e6                     	movq	%r12, %rsi
   26110: 4c 89 ff                     	movq	%r15, %rdi
   26113: ff d1                        	callq	*%rcx
   26115: 44 03 e8                     	addl	%eax, %r13d
   26118: 48 8b 0d d1 26 05 00         	movq	0x526d1(%rip), %rcx     # 0x787f0 <sha256.k20$got>
   2611f: ff d1                        	callq	*%rcx
   26121: 41 03 c5                     	addl	%r13d, %eax
   26124: 48 8b 8c 24 90 01 00 00      	movq	0x190(%rsp), %rcx
   2612c: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26130: 48 8b 05 31 28 05 00         	movq	0x52831(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26137: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   2613c: ff d0                        	callq	*%rax
   2613e: 49 89 c6                     	movq	%rax, %r14
   26141: 48 8b 0d 18 28 05 00         	movq	0x52818(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26148: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   2614d: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   26152: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26157: ff d1                        	callq	*%rcx
   26159: 4c 89 f1                     	movq	%r14, %rcx
   2615c: 03 c1                        	addl	%ecx, %eax
   2615e: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   26163: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   26167: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2616c: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   26171: 48 8b 05 f8 27 05 00         	movq	0x527f8(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26178: 4c 89 f7                     	movq	%r14, %rdi
   2617b: ff d0                        	callq	*%rax
   2617d: 03 d8                        	addl	%eax, %ebx
   2617f: 48 8b 05 d2 27 05 00         	movq	0x527d2(%rip), %rax     # 0x78958 <sha256.ch$got>
   26186: 4c 89 e2                     	movq	%r12, %rdx
   26189: 4c 89 fe                     	movq	%r15, %rsi
   2618c: 4c 89 f7                     	movq	%r14, %rdi
   2618f: ff d0                        	callq	*%rax
   26191: 03 d8                        	addl	%eax, %ebx
   26193: 48 8b 05 5e 26 05 00         	movq	0x5265e(%rip), %rax     # 0x787f8 <sha256.k21$got>
   2619a: ff d0                        	callq	*%rax
   2619c: 03 c3                        	addl	%ebx, %eax
   2619e: 48 8b 8c 24 88 01 00 00      	movq	0x188(%rsp), %rcx
   261a6: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   261aa: 48 8b 05 b7 27 05 00         	movq	0x527b7(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   261b1: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   261b6: ff d0                        	callq	*%rax
   261b8: 48 89 c3                     	movq	%rax, %rbx
   261bb: 48 8b 0d 9e 27 05 00         	movq	0x5279e(%rip), %rcx     # 0x78960 <sha256.maj$got>
   261c2: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   261c7: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   261cc: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   261d1: ff d1                        	callq	*%rcx
   261d3: 49 89 d8                     	movq	%rbx, %r8
   261d6: 41 03 c0                     	addl	%r8d, %eax
   261d9: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   261de: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   261e2: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   261e7: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   261ec: 48 8b 05 7d 27 05 00         	movq	0x5277d(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   261f3: 48 89 df                     	movq	%rbx, %rdi
   261f6: ff d0                        	callq	*%rax
   261f8: 44 03 e0                     	addl	%eax, %r12d
   261fb: 48 8b 0d 56 27 05 00         	movq	0x52756(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26202: 4c 89 fa                     	movq	%r15, %rdx
   26205: 4c 89 f6                     	movq	%r14, %rsi
   26208: 48 89 df                     	movq	%rbx, %rdi
   2620b: ff d1                        	callq	*%rcx
   2620d: 44 03 e0                     	addl	%eax, %r12d
   26210: 48 8b 0d e9 25 05 00         	movq	0x525e9(%rip), %rcx     # 0x78800 <sha256.k22$got>
   26217: ff d1                        	callq	*%rcx
   26219: 41 03 c4                     	addl	%r12d, %eax
   2621c: 48 8b 8c 24 80 01 00 00      	movq	0x180(%rsp), %rcx
   26224: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26228: 48 8b 05 39 27 05 00         	movq	0x52739(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2622f: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26234: ff d0                        	callq	*%rax
   26236: 49 89 c4                     	movq	%rax, %r12
   26239: 48 8b 0d 20 27 05 00         	movq	0x52720(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26240: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   26245: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   2624a: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   2624f: ff d1                        	callq	*%rcx
   26251: 4c 89 e1                     	movq	%r12, %rcx
   26254: 03 c1                        	addl	%ecx, %eax
   26256: 4c 8b 4c 24 28               	movq	0x28(%rsp), %r9
   2625b: 47 8d 24 29                  	leal	(%r9,%r13), %r12d
   2625f: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26264: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   26269: 48 8b 05 00 27 05 00         	movq	0x52700(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26270: 4c 89 e7                     	movq	%r12, %rdi
   26273: ff d0                        	callq	*%rax
   26275: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   26279: 48 8b 0d d8 26 05 00         	movq	0x526d8(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26280: 4c 89 f2                     	movq	%r14, %rdx
   26283: 48 89 de                     	movq	%rbx, %rsi
   26286: 4c 89 e7                     	movq	%r12, %rdi
   26289: ff d1                        	callq	*%rcx
   2628b: 44 03 e8                     	addl	%eax, %r13d
   2628e: 48 8b 0d 73 25 05 00         	movq	0x52573(%rip), %rcx     # 0x78808 <sha256.k23$got>
   26295: ff d1                        	callq	*%rcx
   26297: 41 03 c5                     	addl	%r13d, %eax
   2629a: 48 8b 8c 24 78 01 00 00      	movq	0x178(%rsp), %rcx
   262a2: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   262a6: 48 8b 05 bb 26 05 00         	movq	0x526bb(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   262ad: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   262b2: ff d0                        	callq	*%rax
   262b4: 49 89 c7                     	movq	%rax, %r15
   262b7: 48 8b 0d a2 26 05 00         	movq	0x526a2(%rip), %rcx     # 0x78960 <sha256.maj$got>
   262be: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   262c3: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   262c8: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   262cd: ff d1                        	callq	*%rcx
   262cf: 4c 89 f9                     	movq	%r15, %rcx
   262d2: 03 c1                        	addl	%ecx, %eax
   262d4: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   262d9: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   262dd: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   262e2: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   262e7: 48 8b 05 82 26 05 00         	movq	0x52682(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   262ee: 4c 89 ff                     	movq	%r15, %rdi
   262f1: ff d0                        	callq	*%rax
   262f3: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   262f7: 48 8b 0d 5a 26 05 00         	movq	0x5265a(%rip), %rcx     # 0x78958 <sha256.ch$got>
   262fe: 48 89 da                     	movq	%rbx, %rdx
   26301: 4c 89 e6                     	movq	%r12, %rsi
   26304: 4c 89 ff                     	movq	%r15, %rdi
   26307: ff d1                        	callq	*%rcx
   26309: 44 03 e8                     	addl	%eax, %r13d
   2630c: 48 8b 0d fd 24 05 00         	movq	0x524fd(%rip), %rcx     # 0x78810 <sha256.k24$got>
   26313: ff d1                        	callq	*%rcx
   26315: 41 03 c5                     	addl	%r13d, %eax
   26318: 48 8b 8c 24 70 01 00 00      	movq	0x170(%rsp), %rcx
   26320: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26324: 48 8b 05 3d 26 05 00         	movq	0x5263d(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2632b: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26330: ff d0                        	callq	*%rax
   26332: 49 89 c6                     	movq	%rax, %r14
   26335: 48 8b 0d 24 26 05 00         	movq	0x52624(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2633c: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   26341: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   26346: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   2634b: ff d1                        	callq	*%rcx
   2634d: 4c 89 f2                     	movq	%r14, %rdx
   26350: 03 c2                        	addl	%edx, %eax
   26352: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   26357: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   2635b: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26360: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   26365: 48 8b 05 04 26 05 00         	movq	0x52604(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2636c: 4c 89 f7                     	movq	%r14, %rdi
   2636f: ff d0                        	callq	*%rax
   26371: 03 d8                        	addl	%eax, %ebx
   26373: 48 8b 05 de 25 05 00         	movq	0x525de(%rip), %rax     # 0x78958 <sha256.ch$got>
   2637a: 4c 89 e2                     	movq	%r12, %rdx
   2637d: 4c 89 fe                     	movq	%r15, %rsi
   26380: 4c 89 f7                     	movq	%r14, %rdi
   26383: ff d0                        	callq	*%rax
   26385: 03 d8                        	addl	%eax, %ebx
   26387: 48 8b 05 8a 24 05 00         	movq	0x5248a(%rip), %rax     # 0x78818 <sha256.k25$got>
   2638e: ff d0                        	callq	*%rax
   26390: 03 c3                        	addl	%ebx, %eax
   26392: 48 8b 8c 24 68 01 00 00      	movq	0x168(%rsp), %rcx
   2639a: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2639e: 48 8b 05 c3 25 05 00         	movq	0x525c3(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   263a5: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   263aa: ff d0                        	callq	*%rax
   263ac: 48 89 c3                     	movq	%rax, %rbx
   263af: 48 8b 0d aa 25 05 00         	movq	0x525aa(%rip), %rcx     # 0x78960 <sha256.maj$got>
   263b6: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   263bb: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   263c0: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   263c5: ff d1                        	callq	*%rcx
   263c7: 48 89 d9                     	movq	%rbx, %rcx
   263ca: 03 c1                        	addl	%ecx, %eax
   263cc: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   263d1: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   263d5: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   263da: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   263df: 48 8b 05 8a 25 05 00         	movq	0x5258a(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   263e6: 48 89 df                     	movq	%rbx, %rdi
   263e9: ff d0                        	callq	*%rax
   263eb: 44 03 e0                     	addl	%eax, %r12d
   263ee: 48 8b 0d 63 25 05 00         	movq	0x52563(%rip), %rcx     # 0x78958 <sha256.ch$got>
   263f5: 4c 89 fa                     	movq	%r15, %rdx
   263f8: 4c 89 f6                     	movq	%r14, %rsi
   263fb: 48 89 df                     	movq	%rbx, %rdi
   263fe: ff d1                        	callq	*%rcx
   26400: 44 03 e0                     	addl	%eax, %r12d
   26403: 48 8b 0d 16 24 05 00         	movq	0x52416(%rip), %rcx     # 0x78820 <sha256.k26$got>
   2640a: ff d1                        	callq	*%rcx
   2640c: 41 03 c4                     	addl	%r12d, %eax
   2640f: 48 8b 8c 24 60 01 00 00      	movq	0x160(%rsp), %rcx
   26417: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2641b: 48 8b 05 46 25 05 00         	movq	0x52546(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26422: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26427: ff d0                        	callq	*%rax
   26429: 49 89 c4                     	movq	%rax, %r12
   2642c: 48 8b 0d 2d 25 05 00         	movq	0x5252d(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26433: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   26438: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   2643d: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26442: ff d1                        	callq	*%rcx
   26444: 4c 89 e1                     	movq	%r12, %rcx
   26447: 03 c1                        	addl	%ecx, %eax
   26449: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   2644e: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   26452: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26457: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   2645c: 48 8b 05 0d 25 05 00         	movq	0x5250d(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26463: 4c 89 e7                     	movq	%r12, %rdi
   26466: ff d0                        	callq	*%rax
   26468: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   2646c: 48 8b 0d e5 24 05 00         	movq	0x524e5(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26473: 4c 89 f2                     	movq	%r14, %rdx
   26476: 48 89 de                     	movq	%rbx, %rsi
   26479: 4c 89 e7                     	movq	%r12, %rdi
   2647c: ff d1                        	callq	*%rcx
   2647e: 44 03 e8                     	addl	%eax, %r13d
   26481: 48 8b 0d a0 23 05 00         	movq	0x523a0(%rip), %rcx     # 0x78828 <sha256.k27$got>
   26488: ff d1                        	callq	*%rcx
   2648a: 41 03 c5                     	addl	%r13d, %eax
   2648d: 48 8b 8c 24 58 01 00 00      	movq	0x158(%rsp), %rcx
   26495: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26499: 48 8b 05 c8 24 05 00         	movq	0x524c8(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   264a0: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   264a5: ff d0                        	callq	*%rax
   264a7: 49 89 c7                     	movq	%rax, %r15
   264aa: 48 8b 0d af 24 05 00         	movq	0x524af(%rip), %rcx     # 0x78960 <sha256.maj$got>
   264b1: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   264b6: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   264bb: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   264c0: ff d1                        	callq	*%rcx
   264c2: 4c 89 f9                     	movq	%r15, %rcx
   264c5: 03 c1                        	addl	%ecx, %eax
   264c7: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   264cc: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   264d0: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   264d5: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   264da: 48 8b 05 8f 24 05 00         	movq	0x5248f(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   264e1: 4c 89 ff                     	movq	%r15, %rdi
   264e4: ff d0                        	callq	*%rax
   264e6: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   264ea: 48 8b 0d 67 24 05 00         	movq	0x52467(%rip), %rcx     # 0x78958 <sha256.ch$got>
   264f1: 48 89 da                     	movq	%rbx, %rdx
   264f4: 4c 89 e6                     	movq	%r12, %rsi
   264f7: 4c 89 ff                     	movq	%r15, %rdi
   264fa: ff d1                        	callq	*%rcx
   264fc: 44 03 e8                     	addl	%eax, %r13d
   264ff: 48 8b 0d 2a 23 05 00         	movq	0x5232a(%rip), %rcx     # 0x78830 <sha256.k28$got>
   26506: ff d1                        	callq	*%rcx
   26508: 41 03 c5                     	addl	%r13d, %eax
   2650b: 48 8b 8c 24 50 01 00 00      	movq	0x150(%rsp), %rcx
   26513: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26517: 48 8b 05 4a 24 05 00         	movq	0x5244a(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2651e: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26523: ff d0                        	callq	*%rax
   26525: 49 89 c6                     	movq	%rax, %r14
   26528: 48 8b 0d 31 24 05 00         	movq	0x52431(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2652f: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   26534: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   26539: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   2653e: ff d1                        	callq	*%rcx
   26540: 4c 89 f1                     	movq	%r14, %rcx
   26543: 03 c1                        	addl	%ecx, %eax
   26545: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   2654a: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   2654e: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26553: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   26558: 48 8b 05 11 24 05 00         	movq	0x52411(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2655f: 4c 89 f7                     	movq	%r14, %rdi
   26562: ff d0                        	callq	*%rax
   26564: 03 d8                        	addl	%eax, %ebx
   26566: 48 8b 05 eb 23 05 00         	movq	0x523eb(%rip), %rax     # 0x78958 <sha256.ch$got>
   2656d: 4c 89 e2                     	movq	%r12, %rdx
   26570: 4c 89 fe                     	movq	%r15, %rsi
   26573: 4c 89 f7                     	movq	%r14, %rdi
   26576: ff d0                        	callq	*%rax
   26578: 03 d8                        	addl	%eax, %ebx
   2657a: 48 8b 05 b7 22 05 00         	movq	0x522b7(%rip), %rax     # 0x78838 <sha256.k29$got>
   26581: ff d0                        	callq	*%rax
   26583: 03 c3                        	addl	%ebx, %eax
   26585: 48 8b 8c 24 48 01 00 00      	movq	0x148(%rsp), %rcx
   2658d: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26591: 48 8b 05 d0 23 05 00         	movq	0x523d0(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26598: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   2659d: ff d0                        	callq	*%rax
   2659f: 48 89 c3                     	movq	%rax, %rbx
   265a2: 48 8b 0d b7 23 05 00         	movq	0x523b7(%rip), %rcx     # 0x78960 <sha256.maj$got>
   265a9: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   265ae: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   265b3: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   265b8: ff d1                        	callq	*%rcx
   265ba: 48 89 d9                     	movq	%rbx, %rcx
   265bd: 03 c1                        	addl	%ecx, %eax
   265bf: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   265c4: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   265c8: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   265cd: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   265d2: 48 8b 05 97 23 05 00         	movq	0x52397(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   265d9: 48 89 df                     	movq	%rbx, %rdi
   265dc: ff d0                        	callq	*%rax
   265de: 44 03 e0                     	addl	%eax, %r12d
   265e1: 48 8b 0d 70 23 05 00         	movq	0x52370(%rip), %rcx     # 0x78958 <sha256.ch$got>
   265e8: 4c 89 fa                     	movq	%r15, %rdx
   265eb: 4c 89 f6                     	movq	%r14, %rsi
   265ee: 48 89 df                     	movq	%rbx, %rdi
   265f1: ff d1                        	callq	*%rcx
   265f3: 44 03 e0                     	addl	%eax, %r12d
   265f6: 48 8b 0d 43 22 05 00         	movq	0x52243(%rip), %rcx     # 0x78840 <sha256.k30$got>
   265fd: ff d1                        	callq	*%rcx
   265ff: 41 03 c4                     	addl	%r12d, %eax
   26602: 48 8b 8c 24 40 01 00 00      	movq	0x140(%rsp), %rcx
   2660a: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2660e: 48 8b 05 53 23 05 00         	movq	0x52353(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26615: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   2661a: ff d0                        	callq	*%rax
   2661c: 49 89 c4                     	movq	%rax, %r12
   2661f: 48 8b 0d 3a 23 05 00         	movq	0x5233a(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26626: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   2662b: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   26630: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26635: ff d1                        	callq	*%rcx
   26637: 4c 89 e1                     	movq	%r12, %rcx
   2663a: 03 c1                        	addl	%ecx, %eax
   2663c: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   26641: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   26645: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2664a: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   2664f: 48 8b 05 1a 23 05 00         	movq	0x5231a(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26656: 4c 89 e7                     	movq	%r12, %rdi
   26659: ff d0                        	callq	*%rax
   2665b: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   2665f: 48 8b 0d f2 22 05 00         	movq	0x522f2(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26666: 4c 89 f2                     	movq	%r14, %rdx
   26669: 48 89 de                     	movq	%rbx, %rsi
   2666c: 4c 89 e7                     	movq	%r12, %rdi
   2666f: ff d1                        	callq	*%rcx
   26671: 44 03 e8                     	addl	%eax, %r13d
   26674: 48 8b 0d cd 21 05 00         	movq	0x521cd(%rip), %rcx     # 0x78848 <sha256.k31$got>
   2667b: ff d1                        	callq	*%rcx
   2667d: 41 03 c5                     	addl	%r13d, %eax
   26680: 48 8b 8c 24 38 01 00 00      	movq	0x138(%rsp), %rcx
   26688: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2668c: 48 8b 05 d5 22 05 00         	movq	0x522d5(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26693: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26698: ff d0                        	callq	*%rax
   2669a: 49 89 c7                     	movq	%rax, %r15
   2669d: 48 8b 0d bc 22 05 00         	movq	0x522bc(%rip), %rcx     # 0x78960 <sha256.maj$got>
   266a4: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   266a9: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   266ae: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   266b3: ff d1                        	callq	*%rcx
   266b5: 4d 89 f9                     	movq	%r15, %r9
   266b8: 41 03 c1                     	addl	%r9d, %eax
   266bb: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   266c0: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   266c4: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   266c9: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   266ce: 48 8b 05 9b 22 05 00         	movq	0x5229b(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   266d5: 4c 89 ff                     	movq	%r15, %rdi
   266d8: ff d0                        	callq	*%rax
   266da: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   266de: 48 8b 0d 73 22 05 00         	movq	0x52273(%rip), %rcx     # 0x78958 <sha256.ch$got>
   266e5: 48 89 da                     	movq	%rbx, %rdx
   266e8: 4c 89 e6                     	movq	%r12, %rsi
   266eb: 4c 89 ff                     	movq	%r15, %rdi
   266ee: ff d1                        	callq	*%rcx
   266f0: 44 03 e8                     	addl	%eax, %r13d
   266f3: 48 8b 0d 56 21 05 00         	movq	0x52156(%rip), %rcx     # 0x78850 <sha256.k32$got>
   266fa: ff d1                        	callq	*%rcx
   266fc: 41 03 c5                     	addl	%r13d, %eax
   266ff: 48 8b 8c 24 30 01 00 00      	movq	0x130(%rsp), %rcx
   26707: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2670b: 48 8b 05 56 22 05 00         	movq	0x52256(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26712: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26717: ff d0                        	callq	*%rax
   26719: 49 89 c6                     	movq	%rax, %r14
   2671c: 48 8b 0d 3d 22 05 00         	movq	0x5223d(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26723: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   26728: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   2672d: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26732: ff d1                        	callq	*%rcx
   26734: 4c 89 f1                     	movq	%r14, %rcx
   26737: 03 c1                        	addl	%ecx, %eax
   26739: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   2673e: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   26742: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26747: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   2674c: 48 8b 05 1d 22 05 00         	movq	0x5221d(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26753: 4c 89 f7                     	movq	%r14, %rdi
   26756: ff d0                        	callq	*%rax
   26758: 03 d8                        	addl	%eax, %ebx
   2675a: 48 8b 05 f7 21 05 00         	movq	0x521f7(%rip), %rax     # 0x78958 <sha256.ch$got>
   26761: 4c 89 e2                     	movq	%r12, %rdx
   26764: 4c 89 fe                     	movq	%r15, %rsi
   26767: 4c 89 f7                     	movq	%r14, %rdi
   2676a: ff d0                        	callq	*%rax
   2676c: 03 d8                        	addl	%eax, %ebx
   2676e: 48 8b 05 e3 20 05 00         	movq	0x520e3(%rip), %rax     # 0x78858 <sha256.k33$got>
   26775: ff d0                        	callq	*%rax
   26777: 03 c3                        	addl	%ebx, %eax
   26779: 48 8b 8c 24 28 01 00 00      	movq	0x128(%rsp), %rcx
   26781: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26785: 48 8b 05 dc 21 05 00         	movq	0x521dc(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2678c: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26791: ff d0                        	callq	*%rax
   26793: 48 89 c3                     	movq	%rax, %rbx
   26796: 48 8b 0d c3 21 05 00         	movq	0x521c3(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2679d: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   267a2: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   267a7: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   267ac: ff d1                        	callq	*%rcx
   267ae: 48 89 d9                     	movq	%rbx, %rcx
   267b1: 03 c1                        	addl	%ecx, %eax
   267b3: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   267b8: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   267bc: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   267c1: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   267c6: 48 8b 05 a3 21 05 00         	movq	0x521a3(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   267cd: 48 89 df                     	movq	%rbx, %rdi
   267d0: ff d0                        	callq	*%rax
   267d2: 44 03 e0                     	addl	%eax, %r12d
   267d5: 48 8b 0d 7c 21 05 00         	movq	0x5217c(%rip), %rcx     # 0x78958 <sha256.ch$got>
   267dc: 4c 89 fa                     	movq	%r15, %rdx
   267df: 4c 89 f6                     	movq	%r14, %rsi
   267e2: 48 89 df                     	movq	%rbx, %rdi
   267e5: ff d1                        	callq	*%rcx
   267e7: 44 03 e0                     	addl	%eax, %r12d
   267ea: 48 8b 0d 6f 20 05 00         	movq	0x5206f(%rip), %rcx     # 0x78860 <sha256.k34$got>
   267f1: ff d1                        	callq	*%rcx
   267f3: 41 03 c4                     	addl	%r12d, %eax
   267f6: 48 8b 8c 24 20 01 00 00      	movq	0x120(%rsp), %rcx
   267fe: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26802: 48 8b 05 5f 21 05 00         	movq	0x5215f(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26809: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   2680e: ff d0                        	callq	*%rax
   26810: 49 89 c4                     	movq	%rax, %r12
   26813: 48 8b 0d 46 21 05 00         	movq	0x52146(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2681a: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   2681f: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   26824: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26829: ff d1                        	callq	*%rcx
   2682b: 4c 89 e6                     	movq	%r12, %rsi
   2682e: 03 c6                        	addl	%esi, %eax
   26830: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   26835: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   26839: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2683e: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   26843: 48 8b 05 26 21 05 00         	movq	0x52126(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2684a: 4c 89 e7                     	movq	%r12, %rdi
   2684d: ff d0                        	callq	*%rax
   2684f: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   26853: 48 8b 0d fe 20 05 00         	movq	0x520fe(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2685a: 4c 89 f2                     	movq	%r14, %rdx
   2685d: 48 89 de                     	movq	%rbx, %rsi
   26860: 4c 89 e7                     	movq	%r12, %rdi
   26863: ff d1                        	callq	*%rcx
   26865: 44 03 e8                     	addl	%eax, %r13d
   26868: 48 8b 0d f9 1f 05 00         	movq	0x51ff9(%rip), %rcx     # 0x78868 <sha256.k35$got>
   2686f: ff d1                        	callq	*%rcx
   26871: 41 03 c5                     	addl	%r13d, %eax
   26874: 48 8b 8c 24 18 01 00 00      	movq	0x118(%rsp), %rcx
   2687c: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26880: 48 8b 05 e1 20 05 00         	movq	0x520e1(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26887: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   2688c: ff d0                        	callq	*%rax
   2688e: 49 89 c7                     	movq	%rax, %r15
   26891: 48 8b 0d c8 20 05 00         	movq	0x520c8(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26898: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   2689d: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   268a2: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   268a7: ff d1                        	callq	*%rcx
   268a9: 4c 89 f9                     	movq	%r15, %rcx
   268ac: 03 c1                        	addl	%ecx, %eax
   268ae: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   268b3: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   268b7: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   268bc: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   268c1: 48 8b 05 a8 20 05 00         	movq	0x520a8(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   268c8: 4c 89 ff                     	movq	%r15, %rdi
   268cb: ff d0                        	callq	*%rax
   268cd: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   268d1: 48 8b 0d 80 20 05 00         	movq	0x52080(%rip), %rcx     # 0x78958 <sha256.ch$got>
   268d8: 48 89 da                     	movq	%rbx, %rdx
   268db: 4c 89 e6                     	movq	%r12, %rsi
   268de: 4c 89 ff                     	movq	%r15, %rdi
   268e1: ff d1                        	callq	*%rcx
   268e3: 44 03 e8                     	addl	%eax, %r13d
   268e6: 48 8b 0d 83 1f 05 00         	movq	0x51f83(%rip), %rcx     # 0x78870 <sha256.k36$got>
   268ed: ff d1                        	callq	*%rcx
   268ef: 41 03 c5                     	addl	%r13d, %eax
   268f2: 48 8b 8c 24 10 01 00 00      	movq	0x110(%rsp), %rcx
   268fa: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   268fe: 48 8b 05 63 20 05 00         	movq	0x52063(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26905: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   2690a: ff d0                        	callq	*%rax
   2690c: 49 89 c6                     	movq	%rax, %r14
   2690f: 48 8b 0d 4a 20 05 00         	movq	0x5204a(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26916: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   2691b: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   26920: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26925: ff d1                        	callq	*%rcx
   26927: 4c 89 f1                     	movq	%r14, %rcx
   2692a: 03 c1                        	addl	%ecx, %eax
   2692c: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   26931: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   26935: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2693a: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   2693f: 48 8b 05 2a 20 05 00         	movq	0x5202a(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26946: 4c 89 f7                     	movq	%r14, %rdi
   26949: ff d0                        	callq	*%rax
   2694b: 03 d8                        	addl	%eax, %ebx
   2694d: 48 8b 05 04 20 05 00         	movq	0x52004(%rip), %rax     # 0x78958 <sha256.ch$got>
   26954: 4c 89 e2                     	movq	%r12, %rdx
   26957: 4c 89 fe                     	movq	%r15, %rsi
   2695a: 4c 89 f7                     	movq	%r14, %rdi
   2695d: ff d0                        	callq	*%rax
   2695f: 03 d8                        	addl	%eax, %ebx
   26961: 48 8b 05 10 1f 05 00         	movq	0x51f10(%rip), %rax     # 0x78878 <sha256.k37$got>
   26968: ff d0                        	callq	*%rax
   2696a: 03 c3                        	addl	%ebx, %eax
   2696c: 48 8b 8c 24 08 01 00 00      	movq	0x108(%rsp), %rcx
   26974: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26978: 48 8b 05 e9 1f 05 00         	movq	0x51fe9(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2697f: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26984: ff d0                        	callq	*%rax
   26986: 48 89 c3                     	movq	%rax, %rbx
   26989: 48 8b 0d d0 1f 05 00         	movq	0x51fd0(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26990: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   26995: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   2699a: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   2699f: ff d1                        	callq	*%rcx
   269a1: 48 89 d9                     	movq	%rbx, %rcx
   269a4: 03 c1                        	addl	%ecx, %eax
   269a6: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   269ab: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   269af: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   269b4: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   269b9: 48 8b 05 b0 1f 05 00         	movq	0x51fb0(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   269c0: 48 89 df                     	movq	%rbx, %rdi
   269c3: ff d0                        	callq	*%rax
   269c5: 44 03 e0                     	addl	%eax, %r12d
   269c8: 48 8b 0d 89 1f 05 00         	movq	0x51f89(%rip), %rcx     # 0x78958 <sha256.ch$got>
   269cf: 4c 89 fa                     	movq	%r15, %rdx
   269d2: 4c 89 f6                     	movq	%r14, %rsi
   269d5: 48 89 df                     	movq	%rbx, %rdi
   269d8: ff d1                        	callq	*%rcx
   269da: 44 03 e0                     	addl	%eax, %r12d
   269dd: 48 8b 0d 9c 1e 05 00         	movq	0x51e9c(%rip), %rcx     # 0x78880 <sha256.k38$got>
   269e4: ff d1                        	callq	*%rcx
   269e6: 41 03 c4                     	addl	%r12d, %eax
   269e9: 48 8b 8c 24 00 01 00 00      	movq	0x100(%rsp), %rcx
   269f1: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   269f5: 48 8b 05 6c 1f 05 00         	movq	0x51f6c(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   269fc: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26a01: ff d0                        	callq	*%rax
   26a03: 49 89 c4                     	movq	%rax, %r12
   26a06: 48 8b 0d 53 1f 05 00         	movq	0x51f53(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26a0d: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   26a12: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   26a17: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26a1c: ff d1                        	callq	*%rcx
   26a1e: 4c 89 e1                     	movq	%r12, %rcx
   26a21: 03 c1                        	addl	%ecx, %eax
   26a23: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   26a28: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   26a2c: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26a31: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   26a36: 48 8b 05 33 1f 05 00         	movq	0x51f33(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26a3d: 4c 89 e7                     	movq	%r12, %rdi
   26a40: ff d0                        	callq	*%rax
   26a42: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   26a46: 48 8b 0d 0b 1f 05 00         	movq	0x51f0b(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26a4d: 4c 89 f2                     	movq	%r14, %rdx
   26a50: 48 89 de                     	movq	%rbx, %rsi
   26a53: 4c 89 e7                     	movq	%r12, %rdi
   26a56: ff d1                        	callq	*%rcx
   26a58: 44 03 e8                     	addl	%eax, %r13d
   26a5b: 48 8b 0d 26 1e 05 00         	movq	0x51e26(%rip), %rcx     # 0x78888 <sha256.k39$got>
   26a62: ff d1                        	callq	*%rcx
   26a64: 41 03 c5                     	addl	%r13d, %eax
   26a67: 48 8b 8c 24 f8 00 00 00      	movq	0xf8(%rsp), %rcx
   26a6f: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26a73: 48 8b 05 ee 1e 05 00         	movq	0x51eee(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26a7a: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26a7f: ff d0                        	callq	*%rax
   26a81: 49 89 c7                     	movq	%rax, %r15
   26a84: 48 8b 0d d5 1e 05 00         	movq	0x51ed5(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26a8b: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   26a90: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   26a95: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26a9a: ff d1                        	callq	*%rcx
   26a9c: 4c 89 f9                     	movq	%r15, %rcx
   26a9f: 03 c1                        	addl	%ecx, %eax
   26aa1: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   26aa6: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   26aaa: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26aaf: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   26ab4: 48 8b 05 b5 1e 05 00         	movq	0x51eb5(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26abb: 4c 89 ff                     	movq	%r15, %rdi
   26abe: ff d0                        	callq	*%rax
   26ac0: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   26ac4: 48 8b 0d 8d 1e 05 00         	movq	0x51e8d(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26acb: 48 89 da                     	movq	%rbx, %rdx
   26ace: 4c 89 e6                     	movq	%r12, %rsi
   26ad1: 4c 89 ff                     	movq	%r15, %rdi
   26ad4: ff d1                        	callq	*%rcx
   26ad6: 44 03 e8                     	addl	%eax, %r13d
   26ad9: 48 8b 0d b0 1d 05 00         	movq	0x51db0(%rip), %rcx     # 0x78890 <sha256.k40$got>
   26ae0: ff d1                        	callq	*%rcx
   26ae2: 41 03 c5                     	addl	%r13d, %eax
   26ae5: 48 8b 8c 24 f0 00 00 00      	movq	0xf0(%rsp), %rcx
   26aed: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26af1: 48 8b 05 70 1e 05 00         	movq	0x51e70(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26af8: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26afd: ff d0                        	callq	*%rax
   26aff: 49 89 c6                     	movq	%rax, %r14
   26b02: 48 8b 0d 57 1e 05 00         	movq	0x51e57(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26b09: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   26b0e: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   26b13: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26b18: ff d1                        	callq	*%rcx
   26b1a: 4c 89 f1                     	movq	%r14, %rcx
   26b1d: 03 c1                        	addl	%ecx, %eax
   26b1f: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   26b24: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   26b28: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26b2d: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   26b32: 48 8b 05 37 1e 05 00         	movq	0x51e37(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26b39: 4c 89 f7                     	movq	%r14, %rdi
   26b3c: ff d0                        	callq	*%rax
   26b3e: 03 d8                        	addl	%eax, %ebx
   26b40: 48 8b 05 11 1e 05 00         	movq	0x51e11(%rip), %rax     # 0x78958 <sha256.ch$got>
   26b47: 4c 89 e2                     	movq	%r12, %rdx
   26b4a: 4c 89 fe                     	movq	%r15, %rsi
   26b4d: 4c 89 f7                     	movq	%r14, %rdi
   26b50: ff d0                        	callq	*%rax
   26b52: 03 d8                        	addl	%eax, %ebx
   26b54: 48 8b 05 3d 1d 05 00         	movq	0x51d3d(%rip), %rax     # 0x78898 <sha256.k41$got>
   26b5b: ff d0                        	callq	*%rax
   26b5d: 03 c3                        	addl	%ebx, %eax
   26b5f: 48 8b 8c 24 e8 00 00 00      	movq	0xe8(%rsp), %rcx
   26b67: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26b6b: 48 8b 05 f6 1d 05 00         	movq	0x51df6(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26b72: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26b77: ff d0                        	callq	*%rax
   26b79: 48 89 c3                     	movq	%rax, %rbx
   26b7c: 48 8b 0d dd 1d 05 00         	movq	0x51ddd(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26b83: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   26b88: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   26b8d: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26b92: ff d1                        	callq	*%rcx
   26b94: 49 89 da                     	movq	%rbx, %r10
   26b97: 41 03 c2                     	addl	%r10d, %eax
   26b9a: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   26b9f: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   26ba3: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26ba8: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   26bad: 48 8b 05 bc 1d 05 00         	movq	0x51dbc(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26bb4: 48 89 df                     	movq	%rbx, %rdi
   26bb7: ff d0                        	callq	*%rax
   26bb9: 44 03 e0                     	addl	%eax, %r12d
   26bbc: 48 8b 0d 95 1d 05 00         	movq	0x51d95(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26bc3: 4c 89 fa                     	movq	%r15, %rdx
   26bc6: 4c 89 f6                     	movq	%r14, %rsi
   26bc9: 48 89 df                     	movq	%rbx, %rdi
   26bcc: ff d1                        	callq	*%rcx
   26bce: 44 03 e0                     	addl	%eax, %r12d
   26bd1: 48 8b 0d c8 1c 05 00         	movq	0x51cc8(%rip), %rcx     # 0x788a0 <sha256.k42$got>
   26bd8: ff d1                        	callq	*%rcx
   26bda: 41 03 c4                     	addl	%r12d, %eax
   26bdd: 48 8b 8c 24 e0 00 00 00      	movq	0xe0(%rsp), %rcx
   26be5: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26be9: 48 8b 05 78 1d 05 00         	movq	0x51d78(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26bf0: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26bf5: ff d0                        	callq	*%rax
   26bf7: 49 89 c4                     	movq	%rax, %r12
   26bfa: 48 8b 0d 5f 1d 05 00         	movq	0x51d5f(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26c01: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   26c06: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   26c0b: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26c10: ff d1                        	callq	*%rcx
   26c12: 4c 89 e1                     	movq	%r12, %rcx
   26c15: 03 c1                        	addl	%ecx, %eax
   26c17: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   26c1c: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   26c20: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26c25: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   26c2a: 48 8b 05 3f 1d 05 00         	movq	0x51d3f(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26c31: 4c 89 e7                     	movq	%r12, %rdi
   26c34: ff d0                        	callq	*%rax
   26c36: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   26c3a: 48 8b 0d 17 1d 05 00         	movq	0x51d17(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26c41: 4c 89 f2                     	movq	%r14, %rdx
   26c44: 48 89 de                     	movq	%rbx, %rsi
   26c47: 4c 89 e7                     	movq	%r12, %rdi
   26c4a: ff d1                        	callq	*%rcx
   26c4c: 44 03 e8                     	addl	%eax, %r13d
   26c4f: 48 8b 0d 52 1c 05 00         	movq	0x51c52(%rip), %rcx     # 0x788a8 <sha256.k43$got>
   26c56: ff d1                        	callq	*%rcx
   26c58: 41 03 c5                     	addl	%r13d, %eax
   26c5b: 48 8b 8c 24 d8 00 00 00      	movq	0xd8(%rsp), %rcx
   26c63: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26c67: 48 8b 05 fa 1c 05 00         	movq	0x51cfa(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26c6e: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26c73: ff d0                        	callq	*%rax
   26c75: 49 89 c7                     	movq	%rax, %r15
   26c78: 48 8b 0d e1 1c 05 00         	movq	0x51ce1(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26c7f: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   26c84: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   26c89: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26c8e: ff d1                        	callq	*%rcx
   26c90: 4c 89 f9                     	movq	%r15, %rcx
   26c93: 03 c1                        	addl	%ecx, %eax
   26c95: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   26c9a: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   26c9e: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26ca3: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   26ca8: 48 8b 05 c1 1c 05 00         	movq	0x51cc1(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26caf: 4c 89 ff                     	movq	%r15, %rdi
   26cb2: ff d0                        	callq	*%rax
   26cb4: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   26cb8: 48 8b 0d 99 1c 05 00         	movq	0x51c99(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26cbf: 48 89 da                     	movq	%rbx, %rdx
   26cc2: 4c 89 e6                     	movq	%r12, %rsi
   26cc5: 4c 89 ff                     	movq	%r15, %rdi
   26cc8: ff d1                        	callq	*%rcx
   26cca: 44 03 e8                     	addl	%eax, %r13d
   26ccd: 48 8b 0d dc 1b 05 00         	movq	0x51bdc(%rip), %rcx     # 0x788b0 <sha256.k44$got>
   26cd4: ff d1                        	callq	*%rcx
   26cd6: 41 03 c5                     	addl	%r13d, %eax
   26cd9: 48 8b 8c 24 d0 00 00 00      	movq	0xd0(%rsp), %rcx
   26ce1: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26ce5: 48 8b 05 7c 1c 05 00         	movq	0x51c7c(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26cec: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26cf1: ff d0                        	callq	*%rax
   26cf3: 49 89 c6                     	movq	%rax, %r14
   26cf6: 48 8b 0d 63 1c 05 00         	movq	0x51c63(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26cfd: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   26d02: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   26d07: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26d0c: ff d1                        	callq	*%rcx
   26d0e: 4c 89 f6                     	movq	%r14, %rsi
   26d11: 03 c6                        	addl	%esi, %eax
   26d13: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   26d18: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   26d1c: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26d21: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   26d26: 48 8b 05 43 1c 05 00         	movq	0x51c43(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26d2d: 4c 89 f7                     	movq	%r14, %rdi
   26d30: ff d0                        	callq	*%rax
   26d32: 03 d8                        	addl	%eax, %ebx
   26d34: 48 8b 05 1d 1c 05 00         	movq	0x51c1d(%rip), %rax     # 0x78958 <sha256.ch$got>
   26d3b: 4c 89 e2                     	movq	%r12, %rdx
   26d3e: 4c 89 fe                     	movq	%r15, %rsi
   26d41: 4c 89 f7                     	movq	%r14, %rdi
   26d44: ff d0                        	callq	*%rax
   26d46: 03 d8                        	addl	%eax, %ebx
   26d48: 48 8b 05 69 1b 05 00         	movq	0x51b69(%rip), %rax     # 0x788b8 <sha256.k45$got>
   26d4f: ff d0                        	callq	*%rax
   26d51: 03 c3                        	addl	%ebx, %eax
   26d53: 48 8b 8c 24 c8 00 00 00      	movq	0xc8(%rsp), %rcx
   26d5b: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26d5f: 48 8b 05 02 1c 05 00         	movq	0x51c02(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26d66: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26d6b: ff d0                        	callq	*%rax
   26d6d: 48 89 c3                     	movq	%rax, %rbx
   26d70: 48 8b 0d e9 1b 05 00         	movq	0x51be9(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26d77: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   26d7c: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   26d81: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26d86: ff d1                        	callq	*%rcx
   26d88: 48 89 d9                     	movq	%rbx, %rcx
   26d8b: 03 c1                        	addl	%ecx, %eax
   26d8d: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   26d92: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   26d96: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26d9b: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   26da0: 48 8b 05 c9 1b 05 00         	movq	0x51bc9(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26da7: 48 89 df                     	movq	%rbx, %rdi
   26daa: ff d0                        	callq	*%rax
   26dac: 44 03 e0                     	addl	%eax, %r12d
   26daf: 48 8b 0d a2 1b 05 00         	movq	0x51ba2(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26db6: 4c 89 fa                     	movq	%r15, %rdx
   26db9: 4c 89 f6                     	movq	%r14, %rsi
   26dbc: 48 89 df                     	movq	%rbx, %rdi
   26dbf: ff d1                        	callq	*%rcx
   26dc1: 44 03 e0                     	addl	%eax, %r12d
   26dc4: 48 8b 0d f5 1a 05 00         	movq	0x51af5(%rip), %rcx     # 0x788c0 <sha256.k46$got>
   26dcb: ff d1                        	callq	*%rcx
   26dcd: 41 03 c4                     	addl	%r12d, %eax
   26dd0: 48 8b 8c 24 c0 00 00 00      	movq	0xc0(%rsp), %rcx
   26dd8: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26ddc: 48 8b 05 85 1b 05 00         	movq	0x51b85(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26de3: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26de8: ff d0                        	callq	*%rax
   26dea: 49 89 c4                     	movq	%rax, %r12
   26ded: 48 8b 0d 6c 1b 05 00         	movq	0x51b6c(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26df4: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   26df9: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   26dfe: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   26e03: ff d1                        	callq	*%rcx
   26e05: 4c 89 e1                     	movq	%r12, %rcx
   26e08: 03 c1                        	addl	%ecx, %eax
   26e0a: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   26e0f: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   26e13: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26e18: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   26e1d: 48 8b 05 4c 1b 05 00         	movq	0x51b4c(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26e24: 4c 89 e7                     	movq	%r12, %rdi
   26e27: ff d0                        	callq	*%rax
   26e29: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   26e2d: 48 8b 0d 24 1b 05 00         	movq	0x51b24(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26e34: 4c 89 f2                     	movq	%r14, %rdx
   26e37: 48 89 de                     	movq	%rbx, %rsi
   26e3a: 4c 89 e7                     	movq	%r12, %rdi
   26e3d: ff d1                        	callq	*%rcx
   26e3f: 44 03 e8                     	addl	%eax, %r13d
   26e42: 48 8b 0d 7f 1a 05 00         	movq	0x51a7f(%rip), %rcx     # 0x788c8 <sha256.k47$got>
   26e49: ff d1                        	callq	*%rcx
   26e4b: 41 03 c5                     	addl	%r13d, %eax
   26e4e: 48 8b 8c 24 b8 00 00 00      	movq	0xb8(%rsp), %rcx
   26e56: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26e5a: 48 8b 05 07 1b 05 00         	movq	0x51b07(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26e61: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26e66: ff d0                        	callq	*%rax
   26e68: 49 89 c7                     	movq	%rax, %r15
   26e6b: 48 8b 0d ee 1a 05 00         	movq	0x51aee(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26e72: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   26e77: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   26e7c: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   26e81: ff d1                        	callq	*%rcx
   26e83: 4c 89 f9                     	movq	%r15, %rcx
   26e86: 03 c1                        	addl	%ecx, %eax
   26e88: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   26e8d: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   26e91: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26e96: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   26e9b: 48 8b 05 ce 1a 05 00         	movq	0x51ace(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26ea2: 4c 89 ff                     	movq	%r15, %rdi
   26ea5: ff d0                        	callq	*%rax
   26ea7: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   26eab: 48 8b 0d a6 1a 05 00         	movq	0x51aa6(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26eb2: 48 89 da                     	movq	%rbx, %rdx
   26eb5: 4c 89 e6                     	movq	%r12, %rsi
   26eb8: 4c 89 ff                     	movq	%r15, %rdi
   26ebb: ff d1                        	callq	*%rcx
   26ebd: 44 03 e8                     	addl	%eax, %r13d
   26ec0: 48 8b 0d 09 1a 05 00         	movq	0x51a09(%rip), %rcx     # 0x788d0 <sha256.k48$got>
   26ec7: ff d1                        	callq	*%rcx
   26ec9: 41 03 c5                     	addl	%r13d, %eax
   26ecc: 48 8b 8c 24 b0 00 00 00      	movq	0xb0(%rsp), %rcx
   26ed4: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26ed8: 48 8b 05 89 1a 05 00         	movq	0x51a89(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26edf: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26ee4: ff d0                        	callq	*%rax
   26ee6: 49 89 c6                     	movq	%rax, %r14
   26ee9: 48 8b 0d 70 1a 05 00         	movq	0x51a70(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26ef0: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   26ef5: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   26efa: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   26eff: ff d1                        	callq	*%rcx
   26f01: 4c 89 f1                     	movq	%r14, %rcx
   26f04: 03 c1                        	addl	%ecx, %eax
   26f06: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   26f0b: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   26f0f: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26f14: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   26f19: 48 8b 05 50 1a 05 00         	movq	0x51a50(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26f20: 4c 89 f7                     	movq	%r14, %rdi
   26f23: ff d0                        	callq	*%rax
   26f25: 03 d8                        	addl	%eax, %ebx
   26f27: 48 8b 05 2a 1a 05 00         	movq	0x51a2a(%rip), %rax     # 0x78958 <sha256.ch$got>
   26f2e: 4c 89 e2                     	movq	%r12, %rdx
   26f31: 4c 89 fe                     	movq	%r15, %rsi
   26f34: 4c 89 f7                     	movq	%r14, %rdi
   26f37: ff d0                        	callq	*%rax
   26f39: 03 d8                        	addl	%eax, %ebx
   26f3b: 48 8b 05 96 19 05 00         	movq	0x51996(%rip), %rax     # 0x788d8 <sha256.k49$got>
   26f42: ff d0                        	callq	*%rax
   26f44: 03 c3                        	addl	%ebx, %eax
   26f46: 48 8b 8c 24 a8 00 00 00      	movq	0xa8(%rsp), %rcx
   26f4e: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26f52: 48 8b 05 0f 1a 05 00         	movq	0x51a0f(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26f59: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26f5e: ff d0                        	callq	*%rax
   26f60: 48 89 c3                     	movq	%rax, %rbx
   26f63: 48 8b 0d f6 19 05 00         	movq	0x519f6(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26f6a: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   26f6f: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   26f74: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   26f79: ff d1                        	callq	*%rcx
   26f7b: 48 89 d9                     	movq	%rbx, %rcx
   26f7e: 03 c1                        	addl	%ecx, %eax
   26f80: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   26f85: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   26f89: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   26f8e: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   26f93: 48 8b 05 d6 19 05 00         	movq	0x519d6(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   26f9a: 48 89 df                     	movq	%rbx, %rdi
   26f9d: ff d0                        	callq	*%rax
   26f9f: 44 03 e0                     	addl	%eax, %r12d
   26fa2: 48 8b 0d af 19 05 00         	movq	0x519af(%rip), %rcx     # 0x78958 <sha256.ch$got>
   26fa9: 4c 89 fa                     	movq	%r15, %rdx
   26fac: 4c 89 f6                     	movq	%r14, %rsi
   26faf: 48 89 df                     	movq	%rbx, %rdi
   26fb2: ff d1                        	callq	*%rcx
   26fb4: 44 03 e0                     	addl	%eax, %r12d
   26fb7: 48 8b 0d 22 19 05 00         	movq	0x51922(%rip), %rcx     # 0x788e0 <sha256.k50$got>
   26fbe: ff d1                        	callq	*%rcx
   26fc0: 41 03 c4                     	addl	%r12d, %eax
   26fc3: 48 8b 8c 24 a0 00 00 00      	movq	0xa0(%rsp), %rcx
   26fcb: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   26fcf: 48 8b 05 92 19 05 00         	movq	0x51992(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   26fd6: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26fdb: ff d0                        	callq	*%rax
   26fdd: 49 89 c4                     	movq	%rax, %r12
   26fe0: 48 8b 0d 79 19 05 00         	movq	0x51979(%rip), %rcx     # 0x78960 <sha256.maj$got>
   26fe7: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   26fec: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   26ff1: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   26ff6: ff d1                        	callq	*%rcx
   26ff8: 4c 89 e1                     	movq	%r12, %rcx
   26ffb: 03 c1                        	addl	%ecx, %eax
   26ffd: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   27002: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   27006: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2700b: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   27010: 48 8b 05 59 19 05 00         	movq	0x51959(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   27017: 4c 89 e7                     	movq	%r12, %rdi
   2701a: ff d0                        	callq	*%rax
   2701c: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   27020: 48 8b 0d 31 19 05 00         	movq	0x51931(%rip), %rcx     # 0x78958 <sha256.ch$got>
   27027: 4c 89 f2                     	movq	%r14, %rdx
   2702a: 48 89 de                     	movq	%rbx, %rsi
   2702d: 4c 89 e7                     	movq	%r12, %rdi
   27030: ff d1                        	callq	*%rcx
   27032: 44 03 e8                     	addl	%eax, %r13d
   27035: 48 8b 0d ac 18 05 00         	movq	0x518ac(%rip), %rcx     # 0x788e8 <sha256.k51$got>
   2703c: ff d1                        	callq	*%rcx
   2703e: 41 03 c5                     	addl	%r13d, %eax
   27041: 48 8b 8c 24 98 00 00 00      	movq	0x98(%rsp), %rcx
   27049: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2704d: 48 8b 05 14 19 05 00         	movq	0x51914(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   27054: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   27059: ff d0                        	callq	*%rax
   2705b: 49 89 c7                     	movq	%rax, %r15
   2705e: 48 8b 0d fb 18 05 00         	movq	0x518fb(%rip), %rcx     # 0x78960 <sha256.maj$got>
   27065: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   2706a: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   2706f: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   27074: ff d1                        	callq	*%rcx
   27076: 4d 89 fb                     	movq	%r15, %r11
   27079: 41 03 c3                     	addl	%r11d, %eax
   2707c: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   27081: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   27085: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2708a: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   2708f: 48 8b 05 da 18 05 00         	movq	0x518da(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   27096: 4c 89 ff                     	movq	%r15, %rdi
   27099: ff d0                        	callq	*%rax
   2709b: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   2709f: 48 8b 0d b2 18 05 00         	movq	0x518b2(%rip), %rcx     # 0x78958 <sha256.ch$got>
   270a6: 48 89 da                     	movq	%rbx, %rdx
   270a9: 4c 89 e6                     	movq	%r12, %rsi
   270ac: 4c 89 ff                     	movq	%r15, %rdi
   270af: ff d1                        	callq	*%rcx
   270b1: 44 03 e8                     	addl	%eax, %r13d
   270b4: 48 8b 0d 35 18 05 00         	movq	0x51835(%rip), %rcx     # 0x788f0 <sha256.k52$got>
   270bb: ff d1                        	callq	*%rcx
   270bd: 41 03 c5                     	addl	%r13d, %eax
   270c0: 48 8b 8c 24 90 00 00 00      	movq	0x90(%rsp), %rcx
   270c8: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   270cc: 48 8b 05 95 18 05 00         	movq	0x51895(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   270d3: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   270d8: ff d0                        	callq	*%rax
   270da: 49 89 c6                     	movq	%rax, %r14
   270dd: 48 8b 0d 7c 18 05 00         	movq	0x5187c(%rip), %rcx     # 0x78960 <sha256.maj$got>
   270e4: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   270e9: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   270ee: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   270f3: ff d1                        	callq	*%rcx
   270f5: 4c 89 f1                     	movq	%r14, %rcx
   270f8: 03 c1                        	addl	%ecx, %eax
   270fa: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   270ff: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   27103: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   27108: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   2710d: 48 8b 05 5c 18 05 00         	movq	0x5185c(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   27114: 4c 89 f7                     	movq	%r14, %rdi
   27117: ff d0                        	callq	*%rax
   27119: 03 d8                        	addl	%eax, %ebx
   2711b: 48 8b 05 36 18 05 00         	movq	0x51836(%rip), %rax     # 0x78958 <sha256.ch$got>
   27122: 4c 89 e2                     	movq	%r12, %rdx
   27125: 4c 89 fe                     	movq	%r15, %rsi
   27128: 4c 89 f7                     	movq	%r14, %rdi
   2712b: ff d0                        	callq	*%rax
   2712d: 03 d8                        	addl	%eax, %ebx
   2712f: 48 8b 05 c2 17 05 00         	movq	0x517c2(%rip), %rax     # 0x788f8 <sha256.k53$got>
   27136: ff d0                        	callq	*%rax
   27138: 03 c3                        	addl	%ebx, %eax
   2713a: 48 8b 8c 24 88 00 00 00      	movq	0x88(%rsp), %rcx
   27142: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   27146: 48 8b 05 1b 18 05 00         	movq	0x5181b(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2714d: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   27152: ff d0                        	callq	*%rax
   27154: 48 89 c3                     	movq	%rax, %rbx
   27157: 48 8b 0d 02 18 05 00         	movq	0x51802(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2715e: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   27163: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   27168: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   2716d: ff d1                        	callq	*%rcx
   2716f: 48 89 d9                     	movq	%rbx, %rcx
   27172: 03 c1                        	addl	%ecx, %eax
   27174: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   27179: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   2717d: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   27182: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   27187: 48 8b 05 e2 17 05 00         	movq	0x517e2(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2718e: 48 89 df                     	movq	%rbx, %rdi
   27191: ff d0                        	callq	*%rax
   27193: 44 03 e0                     	addl	%eax, %r12d
   27196: 48 8b 0d bb 17 05 00         	movq	0x517bb(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2719d: 4c 89 fa                     	movq	%r15, %rdx
   271a0: 4c 89 f6                     	movq	%r14, %rsi
   271a3: 48 89 df                     	movq	%rbx, %rdi
   271a6: ff d1                        	callq	*%rcx
   271a8: 44 03 e0                     	addl	%eax, %r12d
   271ab: 48 8b 0d 4e 17 05 00         	movq	0x5174e(%rip), %rcx     # 0x78900 <sha256.k54$got>
   271b2: ff d1                        	callq	*%rcx
   271b4: 41 03 c4                     	addl	%r12d, %eax
   271b7: 48 8b 8c 24 80 00 00 00      	movq	0x80(%rsp), %rcx
   271bf: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   271c3: 48 8b 05 9e 17 05 00         	movq	0x5179e(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   271ca: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   271cf: ff d0                        	callq	*%rax
   271d1: 49 89 c4                     	movq	%rax, %r12
   271d4: 48 8b 0d 85 17 05 00         	movq	0x51785(%rip), %rcx     # 0x78960 <sha256.maj$got>
   271db: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   271e0: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   271e5: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   271ea: ff d1                        	callq	*%rcx
   271ec: 4c 89 e6                     	movq	%r12, %rsi
   271ef: 03 c6                        	addl	%esi, %eax
   271f1: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   271f6: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   271fa: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   271ff: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   27204: 48 8b 05 65 17 05 00         	movq	0x51765(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2720b: 4c 89 e7                     	movq	%r12, %rdi
   2720e: ff d0                        	callq	*%rax
   27210: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   27214: 48 8b 0d 3d 17 05 00         	movq	0x5173d(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2721b: 4c 89 f2                     	movq	%r14, %rdx
   2721e: 48 89 de                     	movq	%rbx, %rsi
   27221: 4c 89 e7                     	movq	%r12, %rdi
   27224: ff d1                        	callq	*%rcx
   27226: 44 03 e8                     	addl	%eax, %r13d
   27229: 48 8b 0d d8 16 05 00         	movq	0x516d8(%rip), %rcx     # 0x78908 <sha256.k55$got>
   27230: ff d1                        	callq	*%rcx
   27232: 41 03 c5                     	addl	%r13d, %eax
   27235: 48 8b 4c 24 78               	movq	0x78(%rsp), %rcx
   2723a: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2723e: 48 8b 05 23 17 05 00         	movq	0x51723(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   27245: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   2724a: ff d0                        	callq	*%rax
   2724c: 49 89 c7                     	movq	%rax, %r15
   2724f: 48 8b 0d 0a 17 05 00         	movq	0x5170a(%rip), %rcx     # 0x78960 <sha256.maj$got>
   27256: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   2725b: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   27260: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   27265: ff d1                        	callq	*%rcx
   27267: 4c 89 f9                     	movq	%r15, %rcx
   2726a: 03 c1                        	addl	%ecx, %eax
   2726c: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   27271: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   27275: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2727a: 48 89 4c 24 18               	movq	%rcx, 0x18(%rsp)
   2727f: 48 8b 05 ea 16 05 00         	movq	0x516ea(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   27286: 4c 89 ff                     	movq	%r15, %rdi
   27289: ff d0                        	callq	*%rax
   2728b: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   2728f: 48 8b 0d c2 16 05 00         	movq	0x516c2(%rip), %rcx     # 0x78958 <sha256.ch$got>
   27296: 48 89 da                     	movq	%rbx, %rdx
   27299: 4c 89 e6                     	movq	%r12, %rsi
   2729c: 4c 89 ff                     	movq	%r15, %rdi
   2729f: ff d1                        	callq	*%rcx
   272a1: 44 03 e8                     	addl	%eax, %r13d
   272a4: 48 8b 0d 65 16 05 00         	movq	0x51665(%rip), %rcx     # 0x78910 <sha256.k56$got>
   272ab: ff d1                        	callq	*%rcx
   272ad: 41 03 c5                     	addl	%r13d, %eax
   272b0: 48 8b 4c 24 70               	movq	0x70(%rsp), %rcx
   272b5: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   272b9: 48 8b 05 a8 16 05 00         	movq	0x516a8(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   272c0: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   272c5: ff d0                        	callq	*%rax
   272c7: 49 89 c6                     	movq	%rax, %r14
   272ca: 48 8b 0d 8f 16 05 00         	movq	0x5168f(%rip), %rcx     # 0x78960 <sha256.maj$got>
   272d1: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   272d6: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   272db: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   272e0: ff d1                        	callq	*%rcx
   272e2: 4c 89 f1                     	movq	%r14, %rcx
   272e5: 03 c1                        	addl	%ecx, %eax
   272e7: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   272ec: 46 8d 34 29                  	leal	(%rcx,%r13), %r14d
   272f0: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   272f5: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   272fa: 48 8b 05 6f 16 05 00         	movq	0x5166f(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   27301: 4c 89 f7                     	movq	%r14, %rdi
   27304: ff d0                        	callq	*%rax
   27306: 03 d8                        	addl	%eax, %ebx
   27308: 48 8b 05 49 16 05 00         	movq	0x51649(%rip), %rax     # 0x78958 <sha256.ch$got>
   2730f: 4c 89 e2                     	movq	%r12, %rdx
   27312: 4c 89 fe                     	movq	%r15, %rsi
   27315: 4c 89 f7                     	movq	%r14, %rdi
   27318: ff d0                        	callq	*%rax
   2731a: 03 d8                        	addl	%eax, %ebx
   2731c: 48 8b 05 f5 15 05 00         	movq	0x515f5(%rip), %rax     # 0x78918 <sha256.k57$got>
   27323: ff d0                        	callq	*%rax
   27325: 03 c3                        	addl	%ebx, %eax
   27327: 48 8b 4c 24 68               	movq	0x68(%rsp), %rcx
   2732c: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   27330: 48 8b 05 31 16 05 00         	movq	0x51631(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   27337: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   2733c: ff d0                        	callq	*%rax
   2733e: 48 89 c3                     	movq	%rax, %rbx
   27341: 48 8b 0d 18 16 05 00         	movq	0x51618(%rip), %rcx     # 0x78960 <sha256.maj$got>
   27348: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   2734d: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   27352: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   27357: ff d1                        	callq	*%rcx
   27359: 48 89 d9                     	movq	%rbx, %rcx
   2735c: 03 c1                        	addl	%ecx, %eax
   2735e: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   27363: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   27367: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   2736c: 48 89 4c 24 28               	movq	%rcx, 0x28(%rsp)
   27371: 48 8b 05 f8 15 05 00         	movq	0x515f8(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   27378: 48 89 df                     	movq	%rbx, %rdi
   2737b: ff d0                        	callq	*%rax
   2737d: 44 03 e0                     	addl	%eax, %r12d
   27380: 48 8b 0d d1 15 05 00         	movq	0x515d1(%rip), %rcx     # 0x78958 <sha256.ch$got>
   27387: 4c 89 fa                     	movq	%r15, %rdx
   2738a: 4c 89 f6                     	movq	%r14, %rsi
   2738d: 48 89 df                     	movq	%rbx, %rdi
   27390: ff d1                        	callq	*%rcx
   27392: 44 03 e0                     	addl	%eax, %r12d
   27395: 48 8b 0d 84 15 05 00         	movq	0x51584(%rip), %rcx     # 0x78920 <sha256.k58$got>
   2739c: ff d1                        	callq	*%rcx
   2739e: 41 03 c4                     	addl	%r12d, %eax
   273a1: 48 8b 4c 24 60               	movq	0x60(%rsp), %rcx
   273a6: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   273aa: 48 8b 05 b7 15 05 00         	movq	0x515b7(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   273b1: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   273b6: ff d0                        	callq	*%rax
   273b8: 49 89 c4                     	movq	%rax, %r12
   273bb: 48 8b 0d 9e 15 05 00         	movq	0x5159e(%rip), %rcx     # 0x78960 <sha256.maj$got>
   273c2: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   273c7: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   273cc: 48 8b 7c 24 28               	movq	0x28(%rsp), %rdi
   273d1: ff d1                        	callq	*%rcx
   273d3: 4c 89 e1                     	movq	%r12, %rcx
   273d6: 03 c1                        	addl	%ecx, %eax
   273d8: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   273dd: 46 8d 24 29                  	leal	(%rcx,%r13), %r12d
   273e1: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   273e6: 48 89 4c 24 08               	movq	%rcx, 0x8(%rsp)
   273eb: 48 8b 05 7e 15 05 00         	movq	0x5157e(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   273f2: 4c 89 e7                     	movq	%r12, %rdi
   273f5: ff d0                        	callq	*%rax
   273f7: 45 8d 2c 07                  	leal	(%r15,%rax), %r13d
   273fb: 48 8b 0d 56 15 05 00         	movq	0x51556(%rip), %rcx     # 0x78958 <sha256.ch$got>
   27402: 4c 89 f2                     	movq	%r14, %rdx
   27405: 48 89 de                     	movq	%rbx, %rsi
   27408: 4c 89 e7                     	movq	%r12, %rdi
   2740b: ff d1                        	callq	*%rcx
   2740d: 44 03 e8                     	addl	%eax, %r13d
   27410: 48 8b 0d 11 15 05 00         	movq	0x51511(%rip), %rcx     # 0x78928 <sha256.k59$got>
   27417: ff d1                        	callq	*%rcx
   27419: 41 03 c5                     	addl	%r13d, %eax
   2741c: 48 8b 4c 24 58               	movq	0x58(%rsp), %rcx
   27421: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   27425: 48 8b 05 3c 15 05 00         	movq	0x5153c(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2742c: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   27431: ff d0                        	callq	*%rax
   27433: 49 89 c7                     	movq	%rax, %r15
   27436: 48 8b 0d 23 15 05 00         	movq	0x51523(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2743d: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   27442: 48 8b 74 24 28               	movq	0x28(%rsp), %rsi
   27447: 48 8b 7c 24 08               	movq	0x8(%rsp), %rdi
   2744c: ff d1                        	callq	*%rcx
   2744e: 4c 89 f9                     	movq	%r15, %rcx
   27451: 03 c1                        	addl	%ecx, %eax
   27453: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   27458: 46 8d 3c 29                  	leal	(%rcx,%r13), %r15d
   2745c: 41 8d 4c 05 00               	leal	(%r13,%rax), %ecx
   27461: 48 89 4c 24 20               	movq	%rcx, 0x20(%rsp)
   27466: 48 8b 05 03 15 05 00         	movq	0x51503(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2746d: 4c 89 ff                     	movq	%r15, %rdi
   27470: ff d0                        	callq	*%rax
   27472: 45 8d 2c 06                  	leal	(%r14,%rax), %r13d
   27476: 48 8b 0d db 14 05 00         	movq	0x514db(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2747d: 48 89 da                     	movq	%rbx, %rdx
   27480: 4c 89 e6                     	movq	%r12, %rsi
   27483: 4c 89 ff                     	movq	%r15, %rdi
   27486: ff d1                        	callq	*%rcx
   27488: 44 03 e8                     	addl	%eax, %r13d
   2748b: 48 8b 0d 9e 14 05 00         	movq	0x5149e(%rip), %rcx     # 0x78930 <sha256.k60$got>
   27492: ff d1                        	callq	*%rcx
   27494: 41 03 c5                     	addl	%r13d, %eax
   27497: 48 8b 4c 24 50               	movq	0x50(%rsp), %rcx
   2749c: 44 8d 34 08                  	leal	(%rax,%rcx), %r14d
   274a0: 48 8b 05 c1 14 05 00         	movq	0x514c1(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   274a7: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   274ac: ff d0                        	callq	*%rax
   274ae: 49 89 c5                     	movq	%rax, %r13
   274b1: 48 8b 0d a8 14 05 00         	movq	0x514a8(%rip), %rcx     # 0x78960 <sha256.maj$got>
   274b8: 48 8b 54 24 28               	movq	0x28(%rsp), %rdx
   274bd: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   274c2: 48 8b 7c 24 20               	movq	0x20(%rsp), %rdi
   274c7: ff d1                        	callq	*%rcx
   274c9: 4c 89 e9                     	movq	%r13, %rcx
   274cc: 03 c1                        	addl	%ecx, %eax
   274ce: 48 8b 4c 24 10               	movq	0x10(%rsp), %rcx
   274d3: 46 8d 2c 31                  	leal	(%rcx,%r14), %r13d
   274d7: 41 8d 0c 06                  	leal	(%r14,%rax), %ecx
   274db: 48 89 4c 24 10               	movq	%rcx, 0x10(%rsp)
   274e0: 48 8b 05 89 14 05 00         	movq	0x51489(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   274e7: 4c 89 ef                     	movq	%r13, %rdi
   274ea: ff d0                        	callq	*%rax
   274ec: 4c 89 6c 24 18               	movq	%r13, 0x18(%rsp)
   274f1: 03 d8                        	addl	%eax, %ebx
   274f3: 48 8b 05 5e 14 05 00         	movq	0x5145e(%rip), %rax     # 0x78958 <sha256.ch$got>
   274fa: 4c 89 e2                     	movq	%r12, %rdx
   274fd: 4c 89 fe                     	movq	%r15, %rsi
   27500: 48 8b 7c 24 18               	movq	0x18(%rsp), %rdi
   27505: ff d0                        	callq	*%rax
   27507: 03 d8                        	addl	%eax, %ebx
   27509: 48 8b 05 28 14 05 00         	movq	0x51428(%rip), %rax     # 0x78938 <sha256.k61$got>
   27510: ff d0                        	callq	*%rax
   27512: 03 c3                        	addl	%ebx, %eax
   27514: 48 8b 4c 24 48               	movq	0x48(%rsp), %rcx
   27519: 44 8d 2c 08                  	leal	(%rax,%rcx), %r13d
   2751d: 48 8b 05 44 14 05 00         	movq	0x51444(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   27524: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   27529: ff d0                        	callq	*%rax
   2752b: 48 89 c3                     	movq	%rax, %rbx
   2752e: 48 8b 0d 2b 14 05 00         	movq	0x5142b(%rip), %rcx     # 0x78960 <sha256.maj$got>
   27535: 48 8b 54 24 08               	movq	0x8(%rsp), %rdx
   2753a: 48 8b 74 24 20               	movq	0x20(%rsp), %rsi
   2753f: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   27544: ff d1                        	callq	*%rcx
   27546: 48 89 d9                     	movq	%rbx, %rcx
   27549: 03 c1                        	addl	%ecx, %eax
   2754b: 48 8b 4c 24 28               	movq	0x28(%rsp), %rcx
   27550: 42 8d 1c 29                  	leal	(%rcx,%r13), %ebx
   27554: 44 03 e8                     	addl	%eax, %r13d
   27557: 48 8b 05 12 14 05 00         	movq	0x51412(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   2755e: 48 89 df                     	movq	%rbx, %rdi
   27561: ff d0                        	callq	*%rax
   27563: 44 03 e0                     	addl	%eax, %r12d
   27566: 48 8b 0d eb 13 05 00         	movq	0x513eb(%rip), %rcx     # 0x78958 <sha256.ch$got>
   2756d: 4c 89 fa                     	movq	%r15, %rdx
   27570: 48 8b 74 24 18               	movq	0x18(%rsp), %rsi
   27575: 48 89 df                     	movq	%rbx, %rdi
   27578: ff d1                        	callq	*%rcx
   2757a: 44 03 e0                     	addl	%eax, %r12d
   2757d: 48 8b 0d bc 13 05 00         	movq	0x513bc(%rip), %rcx     # 0x78940 <sha256.k62$got>
   27584: ff d1                        	callq	*%rcx
   27586: 41 03 c4                     	addl	%r12d, %eax
   27589: 48 8b 4c 24 40               	movq	0x40(%rsp), %rcx
   2758e: 44 8d 34 08                  	leal	(%rax,%rcx), %r14d
   27592: 48 8b 05 cf 13 05 00         	movq	0x513cf(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   27599: 4c 89 ef                     	movq	%r13, %rdi
   2759c: ff d0                        	callq	*%rax
   2759e: 49 89 c4                     	movq	%rax, %r12
   275a1: 48 8b 0d b8 13 05 00         	movq	0x513b8(%rip), %rcx     # 0x78960 <sha256.maj$got>
   275a8: 48 8b 54 24 20               	movq	0x20(%rsp), %rdx
   275ad: 48 8b 74 24 10               	movq	0x10(%rsp), %rsi
   275b2: 4c 89 ef                     	movq	%r13, %rdi
   275b5: ff d1                        	callq	*%rcx
   275b7: 4c 89 e1                     	movq	%r12, %rcx
   275ba: 03 c1                        	addl	%ecx, %eax
   275bc: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   275c1: 46 8d 24 31                  	leal	(%rcx,%r14), %r12d
   275c5: 44 03 f0                     	addl	%eax, %r14d
   275c8: 48 8b 05 a1 13 05 00         	movq	0x513a1(%rip), %rax     # 0x78970 <sha256.sigma1$got>
   275cf: 4c 89 e7                     	movq	%r12, %rdi
   275d2: ff d0                        	callq	*%rax
   275d4: 44 03 f8                     	addl	%eax, %r15d
   275d7: 48 8b 0d 7a 13 05 00         	movq	0x5137a(%rip), %rcx     # 0x78958 <sha256.ch$got>
   275de: 48 8b 54 24 18               	movq	0x18(%rsp), %rdx
   275e3: 48 89 de                     	movq	%rbx, %rsi
   275e6: 4c 89 e7                     	movq	%r12, %rdi
   275e9: ff d1                        	callq	*%rcx
   275eb: 44 03 f8                     	addl	%eax, %r15d
   275ee: 48 8b 0d 53 13 05 00         	movq	0x51353(%rip), %rcx     # 0x78948 <sha256.k63$got>
   275f5: ff d1                        	callq	*%rcx
   275f7: 41 03 c7                     	addl	%r15d, %eax
   275fa: 48 8b 4c 24 38               	movq	0x38(%rsp), %rcx
   275ff: 44 8d 3c 08                  	leal	(%rax,%rcx), %r15d
   27603: 48 8b 05 5e 13 05 00         	movq	0x5135e(%rip), %rax     # 0x78968 <sha256.sigma0$got>
   2760a: 4c 89 f7                     	movq	%r14, %rdi
   2760d: ff d0                        	callq	*%rax
   2760f: 48 89 44 24 08               	movq	%rax, 0x8(%rsp)
   27614: 48 8b 0d 45 13 05 00         	movq	0x51345(%rip), %rcx     # 0x78960 <sha256.maj$got>
   2761b: 48 8b 54 24 10               	movq	0x10(%rsp), %rdx
   27620: 4c 89 ee                     	movq	%r13, %rsi
   27623: 4c 89 f7                     	movq	%r14, %rdi
   27626: ff d1                        	callq	*%rcx
   27628: 48 8b 4c 24 08               	movq	0x8(%rsp), %rcx
   2762d: 03 c1                        	addl	%ecx, %eax
   2762f: 48 8b 4c 24 20               	movq	0x20(%rsp), %rcx
   27634: 41 03 cf                     	addl	%r15d, %ecx
   27637: 41 03 c7                     	addl	%r15d, %eax
   2763a: 48 8b 14 24                  	movq	(%rsp), %rdx
   2763e: 01 42 10                     	addl	%eax, 0x10(%rdx)
   27641: 48 8b 04 24                  	movq	(%rsp), %rax
   27645: 44 01 70 14                  	addl	%r14d, 0x14(%rax)
   27649: 44 01 68 18                  	addl	%r13d, 0x18(%rax)
   2764d: 48 8b 44 24 10               	movq	0x10(%rsp), %rax
   27652: 01 42 1c                     	addl	%eax, 0x1c(%rdx)
   27655: 48 8b 04 24                  	movq	(%rsp), %rax
   27659: 01 48 20                     	addl	%ecx, 0x20(%rax)
   2765c: 48 8b 0c 24                  	movq	(%rsp), %rcx
   27660: 44 01 61 24                  	addl	%r12d, 0x24(%rcx)
   27664: 01 59 28                     	addl	%ebx, 0x28(%rcx)
   27667: 48 8b 4c 24 18               	movq	0x18(%rsp), %rcx
   2766c: 01 48 2c                     	addl	%ecx, 0x2c(%rax)
   2766f: 48 8b 9c 24 30 02 00 00      	movq	0x230(%rsp), %rbx
   27677: 4c 8b a4 24 38 02 00 00      	movq	0x238(%rsp), %r12
   2767f: 4c 8b ac 24 40 02 00 00      	movq	0x240(%rsp), %r13
   27687: 4c 8b b4 24 48 02 00 00      	movq	0x248(%rsp), %r14
   2768f: 4c 8b bc 24 50 02 00 00      	movq	0x250(%rsp), %r15
   27697: 48 81 c4 60 02 00 00         	addq	$0x260, %rsp            # imm = 0x260
   2769e: 48 89 ec                     	movq	%rbp, %rsp
   276a1: 5d                           	popq	%rbp
   276a2: c3                           	retq
