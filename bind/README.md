In this video I'll try to call a shared library created in C through Rust.

It'll be a good exercise to understand how FFI works.

Let's do it!!!

First I'll create a C project

The manager is ready!!!

Now we need to create the plugins. I have a embedded background. So I'll create a button and a LED plugin.

This last function is very important. It's through this function that we will get all the plugin context.

We loaded the plugins with success. This was the first part of project.

Now we can switch to the Rust and load this C library.

Let's see how we can do this!!!

It's a bit complicated but We did it!!!!

The same result by called through Rust. Bye.

TO load external libraries we need to use unsafe
TO make my life easier I'll copy the libraries to the plugin folder