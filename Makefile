all: 
	python3 -m build


clean:
	rm -rf build dist fog05.egg-info
	make -C docs clean

install:
	pip3 install ./dist/fog05-0.3.0a1-py3-none-any.whl 

uninstall:
	pip3 uninstall fog05 -y

doc:
	make -C docs html