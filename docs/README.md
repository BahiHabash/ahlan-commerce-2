# Ahlan Commerce

## تمهيد

اهلا بك في بداية الطريق. مجاز مشروع كبير، ومن الطبيعي ان يكون الدخول اليه
دفعة واحدة مرهقا. لذلك سنبدأ ببناء مشروع اصغر اسمه "اهلا كومرس"؛ مساحة تدريب
هادئة تساعدك على فهم افكارنا الهندسية الاساسية خطوة بخطوة، قبل الانتقال الى
الكود الحقيقي بثقة ووضوح.

وكما قيل في فضل العلم:

> العلم يرفع بيتا لا عماد له  
> والجهل يهدم بيت العز والشرف

الفكرة هنا ليست ان تحفظ شكل الكود، بل ان تتعلم طريقة التفكير: كيف تحول فكرة
غامضة الى متطلبات واضحة، ثم قرار هندسي، ثم تنفيذ صغير قابل للمراجعة، ثم
اختبارات وتوثيق ونشر.

Ahlan Commerce is a guided onboarding book for fresh graduates who know basic
Rust and are preparing to work on Majaz.

You will build one small public project named `ahlan-commerce`. The project is
not a copy of Majaz. It is a smaller learning path that introduces Majaz-style
engineering choices only when you need them.

## How This Book Works

Each chapter is a build contract. It has numbered tasks, and every task must
state:

- `Input`: what you start with
- `Output`: the exact file, API, schema, command, document, or behavior expected
- `Done when`: the check that proves the task is complete

Do not skip ahead. Each chapter depends on outputs from earlier chapters.

## Final Project Shape

By the end, your public GitHub repo should include:

- Rust Axum API
- consistent API error contract with internal root-cause context
- request tracing and runtime diagnostics
- PostgreSQL persistence with Atlas migrations
- SQL-first generated DAL with Cornucopia
- React admin UI with TanStack Router and TanStack Query
- GraphQL API slice with `async-graphql`
- compatibility adapter
- background worker
- Redis-backed storefront cache
- simple storefront HTML rendering
- PRD, ADR, `plan.md`, and `tasks.md`
- unit, integration, and end-to-end tests
- generated OpenAPI and GraphQL docs plus normal docs
- GitHub Actions CI
- Coolify deployment

The concrete contracts are introduced inside the chapter that needs them. Do not
read ahead for schemas or API shapes before the task asks for them.

## Chapter Index

- [Chapter 00 - Start Here](00-start-here/README.md)
- [Chapter 01 - Rust Project Refresher](01-rust-project-refresher/README.md)
- [Chapter 02 - Axum Basics](02-axum-basics/README.md)
- [Chapter 03 - In-Memory Product API](03-in-memory-product-api/README.md)
- [Chapter 03A - Error Handling With rootcause](03a-error-handling-rootcause/README.md)
- [Chapter 03B - Tracing And Observability](03b-tracing-observability/README.md)
- [Chapter 04 - PostgreSQL Basics](04-postgresql-basics/README.md)
- [Chapter 05 - Make For Repeated Commands](05-make-for-repeated-commands/README.md)
- [Chapter 06 - mprocs For Multiple Processes](06-mprocs-for-multiple-processes/README.md)
- [Chapter 07 - SQL-First DAL](07-sql-first-dal/README.md)
- [Chapter 08 - Specs And Tests](08-specs-and-tests/README.md)
- [Chapter 09 - GraphQL Slice](09-graphql-slice/README.md)
- [Chapter 10 - TanStack React Admin UI](10-tanstack-react-admin-ui/README.md)
- [Chapter 11 - Background Worker](11-background-worker/README.md)
- [Chapter 12 - Redis Cache](12-redis-cache/README.md)
- [Chapter 13 - Simple Storefront Rendering](13-simple-storefront-rendering/README.md)
- [Chapter 14 - Generated And Written Docs](14-generated-and-written-docs/README.md)
- [Chapter 15 - AI Workflow With Spec Kit](15-ai-workflow-with-speckit/README.md)
- [Chapter 16 - Compatibility Adapter](16-compatibility-adapter/README.md)
- [Chapter 17 - guard-skills Review](17-guard-skills-review/README.md)
- [Chapter 18 - Prepare For Deployment](18-prepare-for-deployment/README.md)
- [Chapter 19 - Deploy With Coolify](19-deploy-with-coolify/README.md)
- [Chapter 20 - Final Review](20-final-review/README.md)

## Mentor Material

Mentors can audit stack coverage separately in
[mentor/coverage-matrix.md](mentor/coverage-matrix.md). The cargo-manifest scan
notes are in [mentor/cargo-scan-gap-audit.md](mentor/cargo-scan-gap-audit.md).
The learner path should progress chapter by chapter without reading ahead.

## Public References

- [Axum documentation](https://docs.rs/axum/latest/axum/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [PostgreSQL documentation](https://www.postgresql.org/docs/)
- [Atlas documentation](https://atlasgo.io/docs)
- [Cornucopia](https://github.com/cornucopia-rs/cornucopia)
- [tracing](https://docs.rs/tracing/latest/tracing/)
- [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)
- [rootcause](https://docs.rs/rootcause/latest/rootcause/)
- [Redis documentation](https://redis.io/docs/latest/)
- [Spec Kit](https://github.com/github/spec-kit)
- [guard-skills](https://github.com/amElnagdy/guard-skills)
- [Coolify documentation](https://coolify.io/docs)

Next: [Start Here](00-start-here/README.md)
