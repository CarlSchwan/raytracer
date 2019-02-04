BUILDDIR := build
MD       := $(sort $(wildcard *.md))
TEX      := $(sort $(wildcard *.tex))
PDF      := $(TEX:%.tex=$(BUILDDIR)/%.pdf) $(MD:%.md=$(BUILDDIR)/%.pdf)

all: builddir $(PDF)

builddir: 
	@mkdir -p $(BUILDDIR)

$(BUILDDIR)/%.pdf: %.md
	@echo "===> Generating $@ from $< "
	@pandoc -t beamer $< -o $@

$(BUILDDIR)/%.pdf: %.tex
	@echo "===> Generating $@ from $< "
	@latexmk -halt-on-error -pdf -jobname=$(basename $@) $<
	@mv $@ $@.tmp
	@pdfunite $@.tmp $(sort $(wildcard $(basename $(notdir $(subst _$(NAME),,$@)))*.pdf)) $@
	@rm $@.tmp

clean:
	@echo "Clean builddir $(BUILDDIR)"
	@rm -rf $(BUILDDIR)
