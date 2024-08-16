#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/mm.h>
#include <linux/sched.h>
#include <linux/timer.h>
#include <linux/jiffies.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Tu Nombre");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y procesos");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo" // nombre del archivo en /proc

// Esta función se encarga de obtener la información de la memoria y los procesos
static int sysinfo_show(struct seq_file *m, void *v)
{
    struct sysinfo si;
    si_meminfo(&si);

    // Información de memoria
    seq_printf(m, "Total RAM: %lu KB\n", si.totalram * (si.mem_unit / 1024));
    seq_printf(m, "Free RAM: %lu KB\n", si.freeram * (si.mem_unit / 1024));
    seq_printf(m, "Shared RAM: %lu KB\n", si.sharedram * (si.mem_unit / 1024));
    seq_printf(m, "Buffer RAM: %lu KB\n", si.bufferram * (si.mem_unit / 1024));
    seq_printf(m, "Total Swap: %lu KB\n", si.totalswap * (si.mem_unit / 1024));
    seq_printf(m, "Free Swap: %lu KB\n", si.freeswap * (si.mem_unit / 1024));

    // Información de procesos
    seq_printf(m, "\nProcesos:\n");

    // Iterar sobre todos los procesos
    struct task_struct *task;
    struct list_head *list;
    for_each_process(task)
    {
        // Mostrar procesos padre
        if (task->pid == task->real_parent->pid)
        {
            seq_printf(m, "Proceso padre: PID: %d, Nombre: %s\n",
                       task->pid,
                       task->comm);
        }
    }

    seq_printf(m, "\nProceso padre e hijo:\n");

    // Iterar sobre todos los procesos nuevamente para encontrar padres e hijos
    for_each_process(task)
    {
        if (task->real_parent)
        {
            seq_printf(m, "Proceso padre: PID: %d, Nombre: %s\n",
                       task->real_parent->pid,
                       task->real_parent->comm);
            seq_printf(m, "Proceso hijo: PID: %d, Nombre: %s\n",
                       task->pid,
                       task->comm);
        }
    }

    return 0;
}

// Esta función se ejecuta cuando se abre el archivo en /proc
static int sysinfo_open(struct inode *inode, struct file *file)
{
    return single_open(file, sysinfo_show, NULL);
}

// Esta estructura contiene las operaciones a realizar cuando se accede al archivo en /proc
static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};

// Esta macro se encarga de hacer dos cosas:
// 1. Ejecutar la función proc_create, la cual recibe el nombre del archivo a guardar en /proc, permisos,
//    y la estructura con las operaciones a realizar
// 2. Imprimir un mensaje en el log del kernel
static int __init sysinfo_init(void)
{
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo module loaded\n");
    return 0;
}

// Esta macro se encarga de hacer dos cosas:
// 1. Ejecutar la función remove_proc_entry, la cual recibe el nombre del archivo a eliminar de /proc
static void __exit sysinfo_exit(void)
{
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);
