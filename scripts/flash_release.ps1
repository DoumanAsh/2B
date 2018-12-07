cd .\target\thumbv7em-none-eabihf\release\
openocd -f interface/stlink-v2-1.cfg -f target/stm32l4x.cfg -c "program 2B verify reset exit"
cd -
