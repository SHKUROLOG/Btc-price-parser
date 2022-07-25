## Btc-price-parser

Принимает все файлы data/ *.csv и записывает файлы в dist/ *.sql в формате sql

```sql
INSERT INTO
  public.price(ticker, value, "timestamp")
VALUES
  (0, 29203, 1654909517),
  (0, 29218, 1654909512),
  (0, 50000, 1654909563);
```
