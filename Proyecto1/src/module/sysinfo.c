#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/mm.h>
#include <linux/sched.h>
#include <linux/uaccess.h>
#include <linux/string.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Josue Alejandro Perez Benito");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y procesos Docker");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_201712602" // nombre del archivo en /proc

static int sysinfo_show(struct seq_file *m, void *v)
{
    struct sysinfo si;
    struct task_struct *task;
    si_meminfo(&si);

    // Convertir a MB
    unsigned long totalram_mb = (si.totalram * si.mem_unit) / (1024 * 1024);
    unsigned long freeram_mb = (si.freeram * si.mem_unit) / (1024 * 1024);
    unsigned long usedram_mb = totalram_mb - freeram_mb;

    // Mostrar información de la memoria
    seq_printf(m, "{\n");
    seq_printf(m, "\"RAM total (mb)\": %lu,\n", totalram_mb);
    seq_printf(m, "\"RAM libre (mb)\": %lu,\n", freeram_mb);
    seq_printf(m, "\"RAM utilizada (mb)\": %lu,\n", usedram_mb);

    // Mostrar información de procesos relacionados con Docker
    seq_printf(m, "\"docker_processes\": [\n");

    for_each_process(task)
    {
        if (strstr(task->comm, "alto_cpu") || strstr(task->comm, "alto_ram") ||
            strstr(task->comm, "bajo_cpu") || strstr(task->comm, "bajo_ram"))
        {
            long rss = get_mm_rss(task->mm) << PAGE_SHIFT;                              // RSS en bytes
            unsigned long vsz = task->mm ? task->mm->total_vm << (PAGE_SHIFT - 10) : 0; // VSZ en KBs
            unsigned long rss_kb = rss >> 10;                                           // RSS en KBs
            unsigned long cpu_used = task->se.sum_exec_runtime / 1000000;               // CPU usado en ms

            seq_printf(m, "{\n");
            seq_printf(m, "\"pid\": %d,\n", task->pid);
            seq_printf(m, "\"name\": \"%s\",\n", task->comm);
            seq_printf(m, "\"command\": \"%s\",\n", task->comm);
            seq_printf(m, "\"vsz\": %lu KB,\n", vsz);
            seq_printf(m, "\"rss\": %lu KB,\n", rss_kb);
            seq_printf(m, "\"memory_percentage\": %lu,\n", rss_kb * 100 / totalram_mb);
            seq_printf(m, "\"cpu_used\": %lu\n", cpu_used);
            seq_printf(m, "},\n");
        }
    }

    seq_printf(m, "]\n}\n");
    return 0;
}

static int sysinfo_open(struct inode *inode, struct file *file)
{
    return single_open(file, sysinfo_show, NULL);
}

static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};

static int __init sysinfo_init(void)
{
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo module loaded\n");
    return 0;
}

static void __exit sysinfo_exit(void)
{
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);
