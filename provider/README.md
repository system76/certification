Skeleton for a new PlainBox provider
====================================

This is a skeleton PlainBox provider that was generated using
``plainbox startprovider ...``.

It is just the starting point, there is nothing here of value to you
yet. If you know how this works then just remove this file along with
other example content and start working on your new tests,
otherwise, read on.

Inside the ``jobs/`` directory you will find several files that define
a number of "jobs" (more than one job per file actually). A job, in
PlainBox parlance, is the smallest piece of executable test code. Each
job has a name and a number of other attributes.

Jobs can be arranged in test plans - lists that specify which jobs to
run. You can create as many test plans as you need, referring to
arbitrary subsets of your jobs.

Then there are the ``bin/`` and ``data/`` directories. Those are
entirely for custom content you may need. You can put arbitrary
executables in ``bin/``, and those will be available to your job
definitions. Similarly you can keep any data your jobs might need
inside the ``data/`` directory. Referring to that directory at runtime
is a little bit trickier but one of the examples generated in this
skeleton shows how to do that.

Lastly there is the ``manage.py`` script. It requires python3 to run.
It depends on the python3-plainbox Debian package (or just the
PlainBox 0.5 upstream package) installed. This script can automate and
simplify a number of tasks that you will want to do as a test
developer.

Run ``./manage.py --help`` to see what sub-commands are available. You
can additionally pass ``--help`` to each sub command, for example
``./manage.py install --help`` will print the description of the
install command and all the arguments it supports.

That is it for now. You should check out the official documentation
for test authors at
http://plainbox.readthedocs.org/en/latest/author/index.html

If you find bugs or would like to see additional features developed
you can file bugs on the parent project page:
https://bugs.launchpad.net/checkbox/+filebug