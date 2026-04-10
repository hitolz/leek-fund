import { stocks } from "stock-api";  
  
// 使用 Netease 数据源获取 HK00100  
stocks.tencent.getStock("HK00100").then(console.log);
