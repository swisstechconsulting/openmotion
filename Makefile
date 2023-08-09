default: lint test build

lint:

test:

build:

tools: init

init: third-party/stm32cubef4 third-party/stm32g4xx_hal_driver

third-party/stm32g4xx_hal_driver:
	git subtree add --prefix third-party/stm32cubef4 https://github.com/STMicroelectronics/stm32g4xx_hal_driver.git master

third-party/stm32cubef4:
	git subtree add --prefix third-party/stm32cubef4 https://github.com/STMicroelectronics/STM32CubeF4.git master

.PHONY: update
update:
	git subtree pull --prefix third-party/stm32g4xx_hal_driver https://github.com/STMicroelectronics/stm32g4xx_hal_driver.git master
	git subtree pull --prefix third-party/stm32cubef4 https://github.com/STMicroelectronics/STM32CubeF4.git master

.PHONY: clean
clean:

