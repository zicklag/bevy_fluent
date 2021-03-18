# Документация

[🔼](README.md) | [English 🇺🇸](DOCUMENTATION.en-US.md) | **Русский** 🇷🇺

## Использование

## Настройки

Настройки задаются с помощью ресурса `Settings`. Он предоставляет следующие
опции:

- `default_locale` - локаль которая будет использоваться в вашем приложении по
  умолчанию,
- `fallback_locale_chain` - цепочка резервных локалей, которую вы хотите
  использовать в своем приложении,
- `locales_folder` - *директория локалей*.

## Определения

***Корневая директория локалей*** - это корневая директория для всех локалей. По
умолчанию - это `assets/locales/`.

***Директория локали*** - это директория конкретной локали. Например
`assets/locales/ru/`

[***Aссет***][asset] Fluent или просто *ассет* - это любой файл, соответствующий
шаблону `*.ftl`. *Ассет* является атомарной единицей хранения информации на
диске для Fluent.

[***Ассеты локали***][locale-assets] - это коллекция *ассетов*, ассоциированных
с одной локалью.

Каждый ассет представляет собой набор [***сообщений***][message]. *Cообщение* -
это атомарная единица перевода во Fluent.

Каждое *сообщение* имеет [***идентификатор***][identifier].

*Сообщения* (как и [***термы***][term], [***варианты***][variant],
[***аттрибуты***][attribute]) хранят свои значения в виде
[***паттернов***][pattern].

Форматированный *паттерн* называется [***контентом***][content].

[***Запрос***][request] представляет собой запрос на получение соответствующего
заданным параметрам *контента*.

## Структура *корневой директории локалей*

`bevy_fluent` поддерживает два режима сканирования, и, соответственно, два
варианта структуры *корневой директории локалей*: [***явный***][explicit] и
[***неявный***][implicit]. По умолчанию используется *явный* режим. Переключение
режима осуществляется с помощью фичи [`implicit`][implicit].

### Явный режим

В этом режиме каджая *директория локали* должна удовлетворять следующим
критериям: имя папки соответствует [стандарту][unicode_language_identifier],
директория содержит файл `locale.ron`. Файл `locale.ron` представляет собой
список файлов *ассетов*, относящихся к соответствующей локали, другими словами -
это сериализованные *ассеты локали*. Сами файлы *ассетов* располагаются в
иерархии *директории локали*. Файл `locale.ron`, расположенный в *корневой
директории локалей* представляет интерлокаль, которая содержит ресурсы, не
зависящие от языка.

На заметку: файлы `locale.ron` располагаются по аналогии с `mod.rs` в Rust.

Пример иерархии:

```md
locales
    - en-US
        locale.ron
        ...
    - ru
        - ru-RU
            locale.ron
            ...
        - ru-BY
            locale.ron
            ...
    locale.ron
```

Заметьте, что в снепшоте из примера не будет локали `ru`, так как `locales/ru/`
не содержит файл `locale.ron`, потому не считается *директорией локали*. Однако,
если вы создадите файл `locales/ru/locale.ron`, то в результате локаль `ru`
будет добавлена в снепшот.

### Неявный режим

В этом режиме директория или файл на глубине 0 является *директорией локали*,
если ее имя соответствует [стандарту][unicode_language_identifier]. Директории
или файлы на глубине 0, не соответствующие указанному стандарту принадлежат
интерлокали. Директория или файл на глубине больше 0 является *директорией
локали*, если родительская директория также является *директорией локали*, а
локаль родительской директории является надмножеством ее локали. *Ассеты*,
расположенные в иерархии соответствующей *директории локали* относятся к этой
локали.

Пример иерархии:

```md
locales
    - en-US
        ...
    - ru
        - ru-RU
            ...
        - ru-BY
            ...
    locale.ron
```

[asset]: https://github.com/bevyengine/bevy
[attribute]: https://github.com/bevyengine/bevy
[content]: https://github.com/bevyengine/bevy
[explicit]: https://github.com/bevyengine/bevy
[identifier]: https://github.com/bevyengine/bevy
[implicit]: https://github.com/bevyengine/bevy
[locale-assets]: https://github.com/bevyengine/bevy
[message]: https://github.com/bevyengine/bevy
[pattern]: https://github.com/bevyengine/bevy
[request]: https://github.com/bevyengine/bevy
[term]: https://github.com/bevyengine/bevy
[variant]: https://github.com/bevyengine/bevy

[unicode-language-identifier]: http://unicode.org/reports/tr35/#Unicode_language_identifier