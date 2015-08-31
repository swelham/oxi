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
  - [x] self closing tags (this needs to take the ```doctype``` into account)
  - [x] classes (using ```.className``` syntax)
  - [x] attributes (using ```(attr="value")``` syntax)
  - [x] omittable div tag when using classes or attributes
  - [x] option to explictly self close a tag wit the ```tag/``` syntax
  - [x] plain text (using ```| some text``` syntax)
  - [x] pretty print
  - [ ] filter blocks (applied when nested code is discovered under this tag - all nested code is not parsed)
     - [ ] javascript (using standard ```script``` tag)
     - [ ] css (using standard ```style``` tag)
  - [x] code comments
     - [x] templates comments (ignored by parser)
     - [x] html/xml comments (renderd using standard xml comment syntax ```<!-- -->```)
     - [x] nested comments
  - [ ] mixins
  - [ ] inheritance (the ```extends``` feature)
  - [ ] includes (allows injection of content from another template)
  - [ ] data binding
  - [ ] sort this list to group features and add the remaining features
  - [ ] write tests!
