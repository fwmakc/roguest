# roguest

The roguelike rust game

Dev test:

```
cargo run
```

Build:

```
cargo build --release
```

# методы работы с файловой системой для rust

что нужно:

1. методы работы с каталогами:
1.1. проверка существования каталога
1.2. переход или смена каталога
1.3. выбор одного из системных каталогов по-умолчанию (напр, пользователь, документы)
1.4. чтение информации о каталоге
1.5. чтение списка файлов в каталоге
1.6. создание каталога
1.7. копирование каталога
1.8. перемещение каталога
1.9. удаление каталога

2. методы работы с файлами:
1.1. проверка существования файла
1.2. чтение информации о файле
1.3. чтение файла
1.4. создание файла
1.6. копирование файла
1.7. перемещение файла
1.8. удаление файла
1.9. запись в файл

## вариант 1

Для работы с файловой системой в Rust стандартным инструментом остается модуль std::fs, а для путей — std::path. Для работы с системными каталогами (пункт 1.3) общепринятым стандартом является библиотека dirs.

1. Методы работы с каталогами

1.1 Проверка существования: std::path::Path::new("path").is_dir()

1.2 Смена каталога: std::env::set_current_dir("new_path")

1.3 Системные каталоги (нужна библиотека dirs):

Домашний: dirs::home_dir()

Документы: dirs::document_dir()

1.4 Чтение информации (метаданных): std::fs::metadata("path")

1.5 Список файлов: std::fs::read_dir("path") (возвращает итератор)

1.6 Создание каталога:

Одного: std::fs::create_dir("path")

Со всеми родительскими: std::fs::create_dir_all("path/to/dir")

1.7 Копирование каталога: В std::fs нет встроенного рекурсивного копирования. Обычно реализуется через итерацию read_dir или сторонние библиотеки (например, fs_extra).

1.8 Перемещение (переименование): std::fs::rename("old_path", "new_path")

1.9 Удаление:

Пустого: std::fs::remove_dir("path")

Со всем содержимым: std::fs::remove_dir_all("path")

2. Методы работы с файлами

2.1 Проверка существования: std::path::Path::new("file.txt").is_file() или .exists()

2.2 Чтение информации: std::fs::metadata("file.txt")

2.3 Чтение файла:

В строку: std::fs::read_to_string("file.txt")

В вектор байтов: std::fs::read("file.txt")

2.4 Создание файла: std::fs::File::create("file.txt")

2.6 Копирование файла: std::fs::copy("from.txt", "to.txt")

2.7 Перемещение: std::fs::rename("old.txt", "new.txt")

2.8 Удаление: std::fs::remove_file("file.txt")

2.9 Запись в файл:

Простая перезапись: std::fs::write("file.txt", "content")

Через дескриптор: use std::io::Write; → file.write_all(b"content")?

## вариант 2

Вот основные методы работы с файловой системой в Rust с использованием стандартной библиотеки и популярных крейтов:

1. Работа с каталогами

1.1. Проверка существования каталога

```rust
use std::path::Path;

fn directory_exists(path: &str) -> bool {
    Path::new(path).exists() && Path::new(path).is_dir()
}
```

1.2. Переход или смена каталога

```rust
use std::env;

fn change_directory(path: &str) -> std::io::Result<()> {
    env::set_current_dir(path)
}

fn get_current_directory() -> std::io::Result<String> {
    env::current_dir().map(|p| p.display().to_string())
}
```

1.3. Системные каталоги (используем крейт dirs)

```toml
# Cargo.toml
[dependencies]
dirs = "4.0"
```

```rust
use dirs;

fn get_system_directories() {
    if let Some(home) = dirs::home_dir() {
        println!("Домашний каталог: {}", home.display());
    }
    
    if let Some(documents) = dirs::document_dir() {
        println!("Документы: {}", documents.display());
    }
    
    if let Some(desktop) = dirs::desktop_dir() {
        println!("Рабочий стол: {}", desktop.display());
    }
    
    if let Some(downloads) = dirs::download_dir() {
        println!("Загрузки: {}", downloads.display());
    }
}
```

1.4. Чтение информации о каталоге

```rust
use std::fs;

fn get_directory_metadata(path: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(path)?;
    println!("Размер: {} bytes", metadata.len());
    println!("Модифицирован: {:?}", metadata.modified()?);
    println!("Тип: {:?}", metadata.file_type());
    Ok(())
}
```

1.5. Чтение списка файлов в каталоге

```rust
use std::fs;

fn list_directory_contents(path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}

// С фильтрацией по типу
fn list_files_and_dirs(path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            println!("[DIR] {}", path.display());
        } else if path.is_file() {
            println!("[FILE] {}", path.display());
        }
    }
    Ok(())
}
```

1.6. Создание каталога

```rust
use std::fs;

// Создать один каталог
fn create_directory(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)
}

// Создать рекурсивно все каталоги в пути
fn create_directory_all(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)
}
```

1.7. Копирование каталога

```rust
use std::fs;
use std::io;

fn copy_directory(src: &str, dst: &str) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = std::path::Path::new(dst).join(entry.file_name());
        
        if src_path.is_dir() {
            copy_directory(src_path.to_str().unwrap(), dst_path.to_str().unwrap())?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
```

1.8. Перемещение каталога

```rust
use std::fs;

fn move_directory(src: &str, dst: &str) -> std::io::Result<()> {
    fs::rename(src, dst)
}
```

1.9. Удаление каталога

```rust
use std::fs;

// Удалить пустой каталог
fn remove_directory(path: &str) -> std::io::Result<()> {
    fs::remove_dir(path)
}

// Удалить каталог рекурсивно со всем содержимым
fn remove_directory_all(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path)
}
```

2. Работа с файлами

2.1. Проверка существования файла

```rust
use std::path::Path;

fn file_exists(path: &str) -> bool {
    Path::new(path).exists() && Path::new(path).is_file()
}
```

2.2. Чтение информации о файле

```rust
use std::fs;

fn get_file_metadata(path: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(path)?;
    println!("Размер: {} bytes", metadata.len());
    println!("Модифицирован: {:?}", metadata.modified()?);
    println!("Доступ: {:?}", metadata.permissions());
    Ok(())
}
```

2.3. Чтение файла

```rust
use std::fs;
use std::io;

// Чтение всего файла как строки
fn read_file_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

// Чтение всего файла как байтов
fn read_file_to_bytes(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

// Построчное чтение больших файлов
fn read_file_line_by_line(path: &str) -> io::Result<()> {
    use std::io::{BufRead, BufReader};
    
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
```

2.4. Создание файла

```rust
use std::fs;

fn create_file(path: &str) -> std::io::Result<()> {
    fs::File::create(path)?;
    Ok(())
}

// Создание файла с начальным содержимым
fn create_file_with_content(path: &str, content: &str) -> std::io::Result<()> {
    fs::write(path, content)
}
```

2.5. Копирование файла

```rust
use std::fs;

fn copy_file(src: &str, dst: &str) -> std::io::Result<u64> {
    fs::copy(src, dst)
}
```

2.6. Перемещение файла

```rust
use std::fs;

fn move_file(src: &str, dst: &str) -> std::io::Result<()> {
    fs::rename(src, dst)
}
```

2.7. Удаление файла

```rust
use std::fs;

fn delete_file(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)
}
```

2.8. Запись в файл

```rust
use std::fs;
use std::io;

// Запись строки (перезаписывает файл)
fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

// Дописывание в конец файла
fn append_to_file(path: &str, content: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    
    use std::io::Write;
    write!(file, "{}", content)?;
    Ok(())
}

// Блочная запись
fn write_bytes_to_file(path: &str, data: &[u8]) -> io::Result<()> {
    fs::write(path, data)
}
```

Дополнительные примеры

Полный пример с обработкой ошибок

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Создаем каталог
    fs::create_dir_all("test_dir")?;
    
    // Создаем файл
    fs::write("test_dir/file.txt", "Привет, мир!")?;
    
    // Читаем файл
    let content = fs::read_to_string("test_dir/file.txt")?;
    println!("Содержимое файла: {}", content);
    
    // Копируем файл
    fs::copy("test_dir/file.txt", "test_dir/file_copy.txt")?;
    
    // Листинг каталога
    println!("Содержимое каталога:");
    for entry in fs::read_dir("test_dir")? {
        let entry = entry?;
        println!("  {}", entry.file_name().to_string_lossy());
    }
    
    // Удаляем все
    fs::remove_dir_all("test_dir")?;
    
    Ok(())
}
```

Использование крейта walkdir для рекурсивного обхода

```toml
# Cargo.toml
[dependencies]
walkdir = "2"
```

```rust
use walkdir::WalkDir;

fn list_all_files_recursively(path: &str) {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}
```

Эти методы покрывают основные операции с файловой системой в Rust. Для более сложных операций можно использовать дополнительные крейты:

- tempfile - для работы с временными файлами
- ignore - для сложных паттернов игнорирования
- notify - для отслеживания изменений в файловой системе
