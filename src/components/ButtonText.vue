<script setup>
  import { defineProps, defineEmits, inject } from 'vue';
  // 引入sendMessage函数
  import {sendMessage} from '../utils/api.js';
  const props = defineProps({
    options: Array
  });
  
  const stext = inject('stext');

  //编写‘提升表达’函数
  async function improveExpression() {
    //获取当前选中的文本
    const selectedText = stext.value;
    let message = {
      role: 'user',
      content: "输入文本: " + selectedText + ";\n 函数描述：提升输入文本中{}里面的文本的表达效果,并将优化后的文本作为输出文本\n" + "返回：输出文本：\n",
    };
    // add messages
    let messages = [message];
    // use api to improve expression
    // try {
    //   let resp = await sendMessage(messages);
    //   if(resp) {
    //     // update stext
    //     stext.value = "_____________________";
    //     console.log(resp);
    //     // update text
    //   }
    // } catch (error) {
    //   console.error('Error in sending message:', error);
    // }
    {
      // update stext
      stext.value = "_____________________";
      // update text
    }
  }

  // 根据选项来执行操作
  const handleOptionSelected = (option) => {
    switch (option) {
      case '提升表达':
        console.log('提升表达');
        improveExpression();
        break;
      case '修改语气':
        {
          fetch('http://127.0.0.1:8080/api/hello')
          .then(resp => {
            if(!resp.ok) {
              throw new Error("网络响应错误");
            }
            console.log(resp);
          })
          
        }
        break;
      case '抄写':
        console.log('抄写');
        break;
      case '翻译':
        console.log('翻译');
        break;
      case '总结':
        console.log('总结');
        break;
      default:
        break;
    }
  };


  </script>

<template>
    <div class="button-text-container font_black center-text">
      <ul>
        <li v-for="option in options" :key="option" @click="handleOptionSelected(option)" >
          {{ option }}
        </li>
      </ul>
    </div>
  </template>
  
  
  <style scoped>
  .button-text-container {
    background: #fff;
    border: 1px solid #ddd;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    width: 200px;
    /* 其他样式 */
  }
  
  .button-text-container ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
  }
  
  .button-text-container li {
    padding: 10px;
    border-bottom: 1px solid #eee;
    cursor: pointer;
  }
  
  .button-text-container li:last-child {
    border-bottom: none;
  }
  
  .button-text-container li:hover {
    background-color: #f0f0f0;
  }
  </style>
  