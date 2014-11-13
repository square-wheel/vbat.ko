#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/platform_device.h>
#include <linux/power_supply.h>

const int capacity = 42;
static struct platform_device *vbat_platdev;

static enum power_supply_property vbat_props[] =
{
    POWER_SUPPLY_PROP_CAPACITY
};

static int get_vbat_props( struct power_supply* ps
                         , enum power_supply_property pp
                         , union power_supply_propval *v)
{
    int ret = 0;

    switch(pp)
    {
        case POWER_SUPPLY_PROP_CAPACITY:
            v->intval = capacity;
            break;
        default:
            ret = -EINVAL;
            break;
    }
    return ret;
}

static struct power_supply power_supply_vbat =
{
    .properties = vbat_props,
    .num_properties = ARRAY_SIZE(vbat_props),
    .get_property = get_vbat_props,
    .name = "VBAT",
    .type = POWER_SUPPLY_TYPE_BATTERY,
};

static int vbat_init(void)
{
    int ret = 0;
    printk(KERN_INFO "vbat: init\n");
    vbat_platdev = platform_device_register_simple("vbat", 0, NULL, 0);
    ret = power_supply_register(&vbat_platdev->dev, &power_supply_vbat);
    if(!ret)
        printk(KERN_INFO "vbat: done\n");
    return ret;
}

static void vbat_exit(void)
{
    power_supply_unregister(&power_supply_vbat);
    platform_device_unregister(vbat_platdev);
    printk(KERN_INFO "vbat: exit\n");
}

module_init(vbat_init);
module_exit(vbat_exit);

MODULE_LICENSE("GPL");
