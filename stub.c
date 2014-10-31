#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>

static int vbat_init(void)
{
    printk(KERN_INFO "virtual battery: init\n");
    return 0;
}

static void vbat_exit(void)
{
    printk(KERN_INFO "virtual battery: exit\n");
}

module_init(vbat_init);
module_exit(vbat_exit);

MODULE_LICENSE("GPL");
