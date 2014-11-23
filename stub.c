#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/platform_device.h>
#include <linux/power_supply.h>

/* to Rust */

char __morestack[1024];
char _GLOBAL_OFFSET_TABLE_;

long idiv(long a, long b) {
    return a / b;
}

/* from Rust */

extern long rust_percent(long now, long full);


/* actual code */

static struct platform_device *vbat_platdev;

static enum power_supply_property vbat_props[] =
{
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_ENERGY_FULL,
    POWER_SUPPLY_PROP_ENERGY_NOW,
};

static struct power_supply power_supply_vbat;

int summator(enum power_supply_property prop, struct device *dev, long *sum)
{
    union power_supply_propval value;
    struct power_supply *psy = dev_get_drvdata(dev);
    if (strcmp(psy->name, power_supply_vbat.name))
        if (psy->get_property(psy, prop, &value) != -EINVAL)
            *sum += value.intval;
    return 0;
}


int energy_now_summator(struct device *dev, void *energy_now_sum)
{
    return summator(POWER_SUPPLY_PROP_ENERGY_NOW, dev, energy_now_sum);
}

int energy_full_summator(struct device *dev, void *energy_full_sum)
{
    return summator(POWER_SUPPLY_PROP_ENERGY_FULL, dev, energy_full_sum);
}

static int get_vbat_props( struct power_supply* ps
                         , enum power_supply_property pp
                         , union power_supply_propval *v)
{
    union power_supply_propval tempval;
    long tmp;

    switch(pp)
    {
        case POWER_SUPPLY_PROP_CAPACITY:
            power_supply_vbat.get_property(&power_supply_vbat, POWER_SUPPLY_PROP_ENERGY_NOW, &tempval);
            tmp = tempval.intval;
            power_supply_vbat.get_property(&power_supply_vbat, POWER_SUPPLY_PROP_ENERGY_FULL, &tempval);
            v->intval = rust_percent(tmp, tempval.intval);
            break;
        case POWER_SUPPLY_PROP_ENERGY_NOW:
            v->intval = 0;
            class_for_each_device(power_supply_class, NULL, (void*)(&v->intval), energy_now_summator);
            break;
        case POWER_SUPPLY_PROP_ENERGY_FULL:
            v->intval = 0;
            class_for_each_device(power_supply_class, NULL, (void*)(&v->intval), energy_full_summator);
            break;
        default:
            return -EINVAL;
    }
    return 0;
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
