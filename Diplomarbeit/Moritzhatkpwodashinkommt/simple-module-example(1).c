#include <linux/module.h>
#include <linux/printk.h>
#include <linux/fs.h>
#include <linux/uaccess.h>
#include <linux/kernel.h>  

#define BUFFER_SIZE 1024
#define DEVICE_NAME "chartest"

static int major;

enum{
    CDEV_NOT_USED,

    CDEV_EXCLUSIVE_OPEN,
};

static atomic_t already_open = ATOMIC_INIT(CDEV_NOT_USED);

static int device_open(struct inode *indoe, struct file *file)
{
    static int counter = 0;

    if (atomic_cmpxchg(&already_open, CDEV_NOT_USED, CDEV_EXCLUSIVE_OPEN))
        
        return -EBUSY;
    
    pr_info("You opened this %d times\n", counter++);

    return 0;
}

static int device_release(struct inode *inode, struct file *file)
{
    atomic_set(&already_open, CDEV_NOT_USED);

    pr_info("closed\n");

    return 0;
}

static char kernel_buffer[BUFFER_SIZE];

static ssize_t device_read(struct file *file, char __user *buff,
     size_t length, loff_t *offset)
{
    
    return simple_read_from_buffer(buff, length, offset, 
        kernel_buffer, BUFFER_SIZE);
}

static ssize_t device_write(struct file *file, const char __user *buff,
     size_t len, loff_t *offset)
{
    if (len >= BUFFER_SIZE)
        len = BUFFER_SIZE - 1;


    if (copy_from_user(kernel_buffer, buff, len))
        return -EFAULT;
    
    pr_info("successfully written into %s\n", DEVICE_NAME);

    return len;
}


static struct file_operations fops = {
    .owner = THIS_MODULE,
    .open = device_open,
    .release = device_release,
    .read = device_read,
    .write = device_write,
};

static int __init startfunction(void)
{   
    major = register_chrdev(0, DEVICE_NAME, &fops);

    if (major < 0){
        
        pr_alert("failed to register character device\n");
        
        return major;
    }
    
        pr_info("begin %s\n",DEVICE_NAME);
        
        return 0;
}

static void __exit endfunction(void)
{

    unregister_chrdev(major, DEVICE_NAME);

    pr_info("finished %s\n",DEVICE_NAME);
}

module_init(startfunction);
module_exit(endfunction);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Moritz");
MODULE_DESCRIPTION("basic device-module structure");
