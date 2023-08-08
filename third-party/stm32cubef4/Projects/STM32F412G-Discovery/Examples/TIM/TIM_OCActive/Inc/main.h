/**
  ******************************************************************************
  * @file    TIM/TIM_OCActive/Inc/main.h
  * @author  MCD Application Team
  * @brief   Header for main.c module
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2017 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __MAIN_H
#define __MAIN_H

/* Includes ------------------------------------------------------------------*/
#include "stm32f4xx_hal.h"
#include "stm32412g_discovery.h"


/* Exported types ------------------------------------------------------------*/
/* Exported constants --------------------------------------------------------*/
/* User can use this section to tailor TIMx instance used and associated
   resources */
/* Definition for TIMx clock resources */
#define TIMx                           TIM4
#define TIMx_CLK_ENABLE()              __HAL_RCC_TIM4_CLK_ENABLE()

/* Definition for TIMx Channel Pins */
#define TIMx_CHANNEL_GPIO_PORT         __HAL_RCC_GPIOB_CLK_ENABLE();
#define TIMx_GPIO_PORT_CHANNEL1        GPIOB
#define TIMx_GPIO_PORT_CHANNEL2        GPIOB
#define TIMx_GPIO_PORT_CHANNEL3        GPIOB
#define TIMx_GPIO_PORT_CHANNEL4        GPIOB
#define TIMx_GPIO_PIN_CHANNEL1         GPIO_PIN_6
#define TIMx_GPIO_PIN_CHANNEL2         GPIO_PIN_7
#define TIMx_GPIO_PIN_CHANNEL3         GPIO_PIN_8
#define TIMx_GPIO_PIN_CHANNEL4         GPIO_PIN_9
#define TIMx_GPIO_AF_CHANNEL1          GPIO_AF2_TIM4
#define TIMx_GPIO_AF_CHANNEL2          GPIO_AF2_TIM4
#define TIMx_GPIO_AF_CHANNEL3          GPIO_AF2_TIM4
#define TIMx_GPIO_AF_CHANNEL4          GPIO_AF2_TIM4
/* Exported macro ------------------------------------------------------------*/
/* Exported functions ------------------------------------------------------- */

#endif /* __MAIN_H */
