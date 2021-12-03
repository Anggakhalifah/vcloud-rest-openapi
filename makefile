specs = \
27.0.json \
29.0.json \
30.0.json \
31.0.json \
32.0.json \
33.0.json \
34.0.json \
35.0.json \
35.2.json \
36.0.json \
36.1.json \
27.0.yml \
29.0.yml \
30.0.yml \
31.0.yml \
32.0.yml \
33.0.yml \
34.0.yml \
35.0.yml \
35.2.yml \
36.0.yml \
36.1.yml

.PHONY : all
all : $(specs)

website/27.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-repo.vmware.com/vmwb-repository/dcr-public/76f491b4-679c-4e1e-8428-f813d668297a/a2555a1b-22f1-4cca-b481-2a98ab874022/doc/a2555a1b-22f1-4cca-b481-2a98ab874022.zip > $@

website/29.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/ca48e1bb-282b-4fdc-b827-649b819249ed/55142cf1-5bb8-4ab1-8d09-b84f717af5ec/doc/55142cf1-5bb8-4ab1-8d09-b84f717af5ec.zip > $@

website/30.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/7a028e78-bd37-4a6a-8298-9c26c7eeb9aa/09142237-dd46-4dee-8326-e07212fb63a8/doc/09142237-dd46-4dee-8326-e07212fb63a8.zip > $@

website/31.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/f27d65ea-c25b-45ed-9193-c8cc77507622/9a1f04e3-359b-4a19-9c62-7c0fafdfeac8/doc/9a1f04e3-359b-4a19-9c62-7c0fafdfeac8.zip > $@

website/32.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/71e12563-bc11-4d64-821d-92d30f8fcfa1/7424bf8e-aec2-44ad-be7d-b98feda7bae0/doc/7424bf8e-aec2-44ad-be7d-b98feda7bae0.zip > $@

website/33.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/037ccaee-649a-417e-b365-1331034fb28d/1f0fd9eb-0238-4af6-89b5-7e6636f29c65/doc/1f0fd9eb-0238-4af6-89b5-7e6636f29c65.zip > $@

website/34.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/06a3b3da-4c6d-4984-b795-5d64081a4b10/8e47d46b-cfa7-4c06-8b81-4f5548da3102/doc/8e47d46b-cfa7-4c06-8b81-4f5548da3102.zip > $@

website/35.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/e5392f68-0310-4bb0-9622-52adfe664c4c/f4b08a32-8cbc-42a3-8a07-fba714d8d5d1/doc/f4b08a32-8cbc-42a3-8a07-fba714d8d5d1.zip > $@

website/35.2.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/ad96a8e3-043d-4e88-a0ba-87db0965b492/029c9ce7-e5fc-47c7-8003-f4bfa046e6db/doc/029c9ce7-e5fc-47c7-8003-f4bfa046e6db.zip > $@

website/36.0.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/4cb94d1d-d9d0-49cd-b6e5-f4edebaf6753/b83c6907-dc7e-4f3c-9bc5-4060ed512f49/doc/b83c6907-dc7e-4f3c-9bc5-4060ed512f49.zip > $@

website/36.1.zip:
	mkdir -p $(dir $@)
	curl https://vdc-download.vmware.com/vmwb-repository/dcr-public/84726741-c7a2-4445-80e6-f0f941e37595/e8d64305-2e69-45ac-bff5-8f98220c7891/doc/e8d64305-2e69-45ac-bff5-8f98220c7891.zip > $@

./%.json: website/%.zip
	(cd transformer; cargo run --release) < $< > $@

./%.yml: ./%.json
	yq --yaml-output < $< > $@