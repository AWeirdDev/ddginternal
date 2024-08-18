import ddginternal
import ddginternal.ddginternal as internal


html = ddginternal.organic_search("boba shops")["html"]
print(internal.assign_nrj_instances(internal.get_nrj_instances(html)))
