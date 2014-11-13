#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/platform_device.h>
#include <linux/power_supply.h>

static struct platform_device *vbat_platdev;

static enum power_supply_property vbat_props[] =
{
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_ENERGY_FULL,
    POWER_SUPPLY_PROP_ENERGY_NOW,
};

static struct power_supply power_supply_vbat;

int percent(int now, int full)
{
    long long_now  = now;
    long long_full = full;
    return long_now * 100 / long_full;
}

static int get_vbat_props( struct power_supply* ps
                         , enum power_supply_property pp
                         , union power_supply_propval *v)
{
    int ret = 0;
    int i = 0;
    union power_supply_propval tempval;

    int count_bats = 2;
    struct power_supply *bats[] = { power_supply_get_by_name("BAT0")
                                  , power_supply_get_by_name("BAT1")
                                  };
    long sum = 0;
    long tmp;

    switch(pp)
    {
        case POWER_SUPPLY_PROP_CAPACITY:
            power_supply_vbat.get_property(&power_supply_vbat, POWER_SUPPLY_PROP_ENERGY_NOW, &tempval);
            tmp = tempval.intval;
            power_supply_vbat.get_property(&power_supply_vbat, POWER_SUPPLY_PROP_ENERGY_FULL, &tempval);
            v->intval = percent(tmp, tempval.intval);
            break;
        case POWER_SUPPLY_PROP_ENERGY_NOW:
            for(i = 0; i < count_bats; i++)
            {
                if (bats[i]->get_property(bats[i], POWER_SUPPLY_PROP_ENERGY_NOW, &tempval) != -EINVAL)
                    sum += tempval.intval;
            }
            v->intval = sum;
            break;

        case POWER_SUPPLY_PROP_ENERGY_FULL:
            for(i = 0; i < count_bats; i++)
            {
                if (bats[i]->get_property(bats[i], POWER_SUPPLY_PROP_ENERGY_FULL, &tempval) != -EINVAL)
                    sum += tempval.intval;
            }
            v->intval = sum;
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
