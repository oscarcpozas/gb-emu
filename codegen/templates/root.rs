{% import "ops.rs" as macros %}

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::alu;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::*;

{% for i in instructions %}
#[allow(unused_variables)]
fn op_{{i.code}}(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    {%- if i.mnemonic == "NOP" -%}

        {{ macros::nop(i=i) }}

    {%- elif i.mnemonic == "INC" -%}

        {%- if i.bits == 8 -%}
        {{ macros::inc8(i=i) }}
        {%- else -%}
        {{ macros::inc16(i=i) }}
        {%- endif -%}

    {%- elif i.mnemonic == "DEC" -%}

        {%- if i.bits == 8 -%}
        {{ macros::dec8(i=i) }}
        {%- else -%}
        {{ macros::dec16(i=i) }}
        {%- endif -%}

    {%- elif i.mnemonic == "LD" -%}

        {{ macros::ld(i=i) }}

    {%- elif i.mnemonic == "LDD" -%}

        {{ macros::ld(i=i) }}
        cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    {%- elif i.mnemonic == "LDI" -%}

        {{ macros::ld(i=i) }}
        cpu.set_hl(cpu.get_hl().wrapping_add(1));

    {%- elif i.mnemonic == "LDHL" -%}

        {{ macros::ldhl(i=i) }}

    {%- elif i.mnemonic == "ADD" -%}

        {%- if i.code == "0xE8" -%}
            {{ macros::addsp(i=i) }}
        {%- else -%}
            {%- if i.bits == 8 -%}
            {{ macros::add8(i=i) }}
            {%- else -%}
            {{ macros::add16(i=i) }}
            {%- endif -%}
        {%- endif -%}

    {%- elif i.mnemonic == "SUB" -%}

        {{ macros::sub(i=i) }}

    {%- elif i.mnemonic == "ADC" -%}

        {{ macros::adc(i=i) }}

    {%- elif i.mnemonic == "SBC" -%}

        {{ macros::sbc(i=i) }}

    {%- elif i.mnemonic == "AND" -%}

        {{ macros::and(i=i) }}

    {%- elif i.mnemonic == "OR" -%}

        {{ macros::or(i=i) }}

    {%- elif i.mnemonic == "XOR" -%}

        {{ macros::xor(i=i) }}

    {%- elif i.mnemonic == "CP" -%}

        {{ macros::cp(i=i) }}

    {%- elif i.mnemonic == "PUSH" -%}

        {{ macros::push(i=i) }}

    {%- elif i.mnemonic == "POP" -%}

        {{ macros::pop(i=i) }}

    {%- elif i.mnemonic == "SWAP" -%}

        {{ macros::swap(i=i) }}

    {%- elif i.mnemonic == "DAA" -%}
        {{ macros::daa(i=i) }}

    {%- elif i.mnemonic == "CPL" -%}

        {{ macros::cpl(i=i) }}

    {%- elif i.mnemonic == "CCF" -%}

        {{ macros::ccf(i=i) }}

    {%- elif i.mnemonic == "SCF" -%}

        {{ macros::scf(i=i) }}

    {%- elif i.mnemonic == "EI" -%}

        {{ macros::ei(i=i) }}

    {%- elif i.mnemonic == "DI" -%}

        {{ macros::di(i=i) }}
        
    {%- elif i.mnemonic == "HALT" -%}
        
        {{ macros::halt(i=i) }}
        
    {%- elif i.mnemonic == "STOP" -%}

        {{ macros::stop(i=i) }}

    {%- elif i.mnemonic == "RLC" -%}

        {{ macros::rlc(i=i) }}

    {%- elif i.mnemonic == "RLCA" -%}

        {{ macros::rlca(i=i) }}

    {%- elif i.mnemonic == "RL" -%}

        {{ macros::rl(i=i) }}

    {%- elif i.mnemonic == "RLA" -%}

        {{ macros::rla(i=i) }}

    {%- elif i.mnemonic == "RRC" -%}

        {{ macros::rrc(i=i) }}

    {%- elif i.mnemonic == "RRCA" -%}

        {{ macros::rrca(i=i) }}

    {%- elif i.mnemonic == "RR" -%}

        {{ macros::rr(i=i) }}

    {%- elif i.mnemonic == "RRA" -%}

        {{ macros::rra(i=i) }}

    {%- elif i.mnemonic == "SLA" -%}

        {{ macros::sla(i=i) }}

    {%- elif i.mnemonic == "SRA" -%}

        {{ macros::sra(i=i) }}

    {%- elif i.mnemonic == "SRL" -%}

        {{ macros::srl(i=i) }}

    {%- elif i.mnemonic == "BIT" -%}

        {{ macros::bit(i=i) }}

    {%- elif i.mnemonic == "SET" -%}

        {{ macros::set(i=i) }}

    {%- elif i.mnemonic == "RES" -%}

        {{ macros::res(i=i) }}

    {%- elif i.mnemonic == "JR" -%}

        {%- if i.cycles | length == 2 -%}
        {{ macros::jrif(i=i) }}
        {%- else -%}
        {{ macros::jr(i=i) }}
        {%- endif -%}

    {%- elif i.mnemonic == "JP" -%}

        {%- if i.cycles | length == 2 -%}
        {{ macros::jpif(i=i) }}
        {%- else -%}
        {{ macros::jp(i=i) }}
        {%- endif -%}

    {%- elif i.mnemonic == "CALL" -%}

        {%- if i.cycles | length == 2 -%}
        {{ macros::callif(i=i) }}
        {%- else -%}
        {{ macros::call(i=i) }}
        {%- endif -%}

    {%- elif i.mnemonic == "RST" -%}

        {{ macros::rst(i=i) }}

    {%- elif i.mnemonic == "RET" -%}

        {%- if i.cycles | length == 2 -%}
        {{ macros::retif(i=i) }}
        {%- else -%}
        {{ macros::ret(i=i) }}
        {%- endif -%}

    {%- elif i.mnemonic == "RETI" -%}

        {{ macros::reti(i=i) }}

    {%- endif -%}

    {{ i.flags.Z | setflag(flg="z") }}
    {{ i.flags.N | setflag(flg="n") }}
    {{ i.flags.H | setflag(flg="h") }}
    {{ i.flags.C | setflag(flg="c") }}

    ({{i.cycles[0]}}, {{i.bytes}})
}
{% endfor %}

pub fn decode(code: u16, arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    match code {
        {% for i in instructions -%}
        {{i.code}} => op_{{i.code}}(arg, cpu, mmu),
        {% endfor -%}
        _ => panic!("Invalid opcode: {:04x}: {:04x}", cpu.get_pc(), code),
    }
}