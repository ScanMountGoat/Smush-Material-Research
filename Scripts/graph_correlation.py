import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import os
import sys

param_names = ['BlendState0', 'CustomBoolean0', 'CustomBoolean1', 'CustomBoolean10', 'CustomBoolean11', 'CustomBoolean12', 'CustomBoolean2', 'CustomBoolean3', 'CustomBoolean4', 'CustomBoolean5', 'CustomBoolean6', 'CustomBoolean7', 'CustomBoolean8', 'CustomBoolean9', 'CustomFloat0', 'CustomFloat1', 'CustomFloat10', 'CustomFloat11', 'CustomFloat12', 'CustomFloat16', 'CustomFloat17', 'CustomFloat18', 'CustomFloat19', 'CustomFloat4', 'CustomFloat6', 'CustomFloat8', 'CustomVector0', 'CustomVector1', 'CustomVector10', 'CustomVector11', 'CustomVector13', 'CustomVector14', 'CustomVector15', 'CustomVector16', 'CustomVector18', 'CustomVector19', 'CustomVector2', 'CustomVector20', 'CustomVector21', 'CustomVector22', 'CustomVector23', 'CustomVector24', 'CustomVector27', 'CustomVector29', 'CustomVector3', 'CustomVector30', 'CustomVector31', 'CustomVector32', 'CustomVector33', 'CustomVector34', 'CustomVector35', 'CustomVector37', 'CustomVector38', 'CustomVector39', 'CustomVector4', 'CustomVector40', 'CustomVector42', 'CustomVector43', 'CustomVector44', 'CustomVector45', 'CustomVector46', 'CustomVector47', 'CustomVector5', 'CustomVector6', 'CustomVector7', 'CustomVector8', 'CustomVector9', 'RasterizerState0', 'Sampler0', 'Sampler1', 'Sampler10', 'Sampler11', 'Sampler12', 'Sampler13', 'Sampler14', 'Sampler16', 'Sampler2', 'Sampler3', 'Sampler4', 'Sampler5', 'Sampler6', 'Sampler7', 'Sampler8', 'Sampler9', 'Texture0', 'Texture1', 'Texture10', 'Texture11', 'Texture12', 'Texture13', 'Texture14', 'Texture16', 'Texture2', 'Texture3', 'Texture4', 'Texture5', 'Texture6', 'Texture7', 'Texture8', 'Texture9', 'UvTransform0', 'UvTransform1', 'UvTransform14', 'UvTransform2', 'UvTransform3', 'UvTransform4', 'UvTransform5', 'UvTransform6', 'UvTransform7', 'UvTransform9']

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print(f'Usage: {sys.argv[0]} <source file>')
        exit(1)

    source_file = os.path.abspath(sys.argv[1])
    df = pd.read_csv(source_file)

    # Limit the number of columns displayed at once.
    # df = df[[name for name in param_names if ('Texture' in name or 'Sampler' in name)]]

    # Plot a correlation heatmap.
    plt.figure()
    labels = [name.replace('Custom','') for name in df.columns]
    sns.heatmap(df.corr(),vmin=-1.0,vmax=1.0, linewidth=0.25, cmap='coolwarm', xticklabels=labels, yticklabels=labels)
    plt.show()