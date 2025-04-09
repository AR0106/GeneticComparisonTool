# GCF File Format Spec

## GCF Header Information
The header of a GCF file may contain the following information:

- **File Version**: The version of the GCF file format. 1 Byte
- **Genome Name**: The name of the genome. 32 Bytes
- **Organism**: The organism from which the genome was derived. 32 Bytes
- **Date Created**: The date the file was created. 4 Bytes
- **Creator**: The name or identifier of the person or system that created the file. 32 Bytes
- **Description**: A brief description of the contents or purpose of the file. 128 Bytes
- **Source**: The source or reference from which the genome data was obtained. 32 Bytes
- **Comments**: Any additional comments or notes about the file. 249 Bytes

**Total Header Size:** 512 Bytes

### Header Table
|Section|Size (Bytes)|Offset|
|--------|------------|------|
|File Version|1|0|
|Genome Name|32|1|
|Organism|32|33|
|Date Created|4|65|
|Creator|32|69|
|Description|128|101|
|Source|32|229|
|Comments|249|261|
|Data|*|513|

## GCF Sequence
The GCF Sequence is the portion that contains the actual sequence data

### Encoding Reference
| Character | Octal Representation | Description |
| -------------- | --------------- | --------------- |
| A | 0o0 | 'A' Nucleotide |
| T | 0o1 | 'T' Nucleotide |
| C | 0o2 | 'C' Nucleotide |
| G | 0o3 | 'G' Nucleotide |
| : | 0o3 | Mark end of sequence |
| ' ' | 0o3 | Mark end of data entry |
| N | 0o6 | Unknown nucleic acid residue |
| X | 0o7 | Unknown amino acid residue |
| U | 0o10 | 'U' Nucleotide |
