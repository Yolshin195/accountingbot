# Accounting Bot Ecosystem 📊🤖

Инфраструктура для ведения учета, включающая в себя **Telegram-бота**, **API-сервис**, **Web-клиент** и базу данных **PostgreSQL**. Проект полностью контейнеризирован и готов к multi-platform развертыванию.



## 🏗 Архитектура системы

Проект состоит из четырех основных компонентов, описанных в `docker-compose.yml`:
1.  **telegram-bot**: Основной интерфейс взаимодействия пользователей на Rust.
2.  **api**: Бекэнд-сервис, обрабатывающий бизнес-логику и запросы к БД.
3.  **client**: Веб-интерфейс (доступен через Nginx с поддержкой SSL).
4.  **db**: Реляционная база данных PostgreSQL 16.


## 🛠 Быстрый старт с Docker

Для запуска всей системы целиком вам понадобится установленный Docker и Docker Compose.

### 1. Подготовка окружения
Создайте файл `.env-docker` (и `.env` для локальной разработки) на основе ваших данных:
```env
DATABASE_URL=postgresql://postgres:postgres@db:5432/accounting_db
RUST_LOG=debug
JWT_SECRET=your_secret_key
TELOXIDE_TOKEN=your_telegram_bot_token
TELEGRAM_BOT_CLIENT_ID=telegram_bot
TELEGRAM_BOT_SECRET=your_password
```

### 2. Генерация SSL сертификатов (для локальной разработки)
Для работы клиентской части по HTTPS используйте команду из `Makefile`:
```bash
make gen-cert
make link-dev-certs
```

### 3. Запуск системы
```bash
make up
```
*Система будет доступна по адресам:*
* **API**: `http://localhost:8888`
* **Web Client**: `https://localhost` (или `http://localhost`)
* **DB**: `localhost:5445` (внешний порт)

---

## 📜 Команды Makefile

В проекте настроен мощный `Makefile` для автоматизации рутинных задач:

| Команда | Описание |
| :--- | :--- |
| `make build-local` | Быстрая сборка Docker-образа бота локально. |
| `make build-push` | Multi-platform сборка (`amd64`, `arm64`) и пуш в Docker Hub. |
| `make rebuild` | Полный перезапуск `docker-compose` с пересборкой образа бота. |
| `make gen-cert` | Создание локального Root CA и SSL-сертификатов для `localhost`. |
| `make logs` | Просмотр логов всех контейнеров в реальном времени. |
| `make stop` | Остановка и удаление всех контейнеров проекта. |


## 🐳 Docker и CI/CD

Проект поддерживает **Multi-arch сборку**. Это значит, что образы можно собирать одновременно для обычных серверов (`amd64`) и для ARM-процессоров (например, Apple Silicon или Raspberry Pi).

**Конфигурация Dockerfile:**
* Базовый образ: `rust:trixie` (Debian Trixie).
* Особенности: Предварительная установка `libssl-dev` и инструментов сборки для работы с `sqlx` и `argon2`.
* Сборка: Оптимизированный `--release` билд.


## 🛠 Технологии в стеке

* **Язык**: Rust 2024
* **База данных**: PostgreSQL 16
* **API**: REST API (на базе `reqwest` в боте)
* **Прокси/Веб**: Nginx (в контейнере `client`)
* **Безопасность**: OpenSSL (SSL/TLS сертификаты), JWT для авторизации.
