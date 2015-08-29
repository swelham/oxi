# oxi

## feature to do list

  - [ ] document validation
    - [x] must start with ```doctype``` or ```extends```
    - [ ] indentation should be consitant (this needs some thought about how strict this needs to be)
    - [ ] ```xml``` documents can only use basic tags and attributes
    - [ ] ```json``` validation considerations
  - [ ] fully implement support for all ```doctype``` options
     - [ ] html
     - [x] xml
     - [ ] json
  - [x] basic tags
  - [x] nested tags
  - [ ] self closing tags (this needs to take the ```doctype``` into account)
  - [x] classes (using ```.className``` syntax)
  - [x] attributes (using ```(attr="value")``` syntax)
  - [x] omittable div tag when using classes or attributes
  - [ ] option to explictly self close a tag wit the ```tag/``` syntax
  - [ ] plain text (using ```| some text``` syntax)
  - [ ] pretty print
  - [ ] sort this list to group features and add the remaining features
  - [ ] write tests!
