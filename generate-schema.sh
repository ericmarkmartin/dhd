#!/usr/bin/env sh

graphql-client introspect-schema http://localhost:8000/graphql > dhd_client/graphql/hashlist_schema.json
