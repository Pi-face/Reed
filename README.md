# 🌿 Reed

**Reed** is a lightweight, embeddable programming language designed for intuitive and declarative data transformation. Its MVP focuses on parsing and transforming JSON into CSV or filtered JSON, making it perfect for quick ETL tasks, developer tooling, and data pre-processing pipelines.

---

## 🚀 MVP Overview

Reed is currently in its Minimum Viable Product (MVP) stage, focused on supporting simple but powerful transformation operations for structured data.

### ✅ Supported Features

- Load JSON files into named variables
- Filter array-based JSON data using simple conditions
- Select and project specific fields from objects
- Export the result as a CSV or transformed JSON

### 🔤 Example Script

```reed
load "data.json" as data
filter data where age > 25
select data.name, data.email
save data to "output.csv"

```

### Rust means obligatory miku pic.
![image](https://github.com/user-attachments/assets/3edaec7a-89d5-47e2-8735-7c0c908848e1)

