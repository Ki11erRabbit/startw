# startw
A replacement for startx for Wayland.

This is a replacement for startx, however it does not behave the same way that startx does for X.

For example, if there is a .winitrc file at the user's home directory then it will always run that. This is
because, unlike startx, you can pass arguments into startw. 

This is to allow for scripting the behavoir.

## Examples
.winitrc:

```
case $1 in
X) startx;;
dwl) dwl;;
sway) sway;;
esac
```
This would allow the user to be able to choose Wayland compositor they wish to launch including running startx if 
desired when starting with `startw`.

```
startw dwl -s 'dwlb'
```

Running `startw` without a `.winitrc` will run the first commandline argument with all following arguments as its
arguments.

## Installation
The dependancies are to have the Rust toolchain installed

Then the steps are

- `git clone https://github.com/Ki11erRabbit/startw/`
- `cd startw`
- `make`
- `sudo make install`

