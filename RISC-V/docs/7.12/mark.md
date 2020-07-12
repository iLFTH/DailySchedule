User & 2+ privileged modes (hierarchical)
- User (U-mode), normal and virtualized* (lowest privileges)
- Supervisor(S-mode), normal and virtualized*
- Machine (M-mode) (highest privileges)**  

Supported combinations of modes:
- M (simple embedded systems)
- M, U (embedded systems with protection)
- M, S, U (systems running Unix-like operating systems)
- M, [V]S, [V]U (systems running multiple Oses)

CSRs Control/Status Regs  
- Direct address mode only
- accesses by lower privilege modes will trap
